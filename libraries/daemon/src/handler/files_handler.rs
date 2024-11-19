use std::sync::Arc;

use eyre::OptionExt;
use tokio::{io::AsyncWriteExt, sync::mpsc::Receiver, task::JoinHandle};
use zenoh::{query::Query, Session};

use crate::{queries::DaemonReply, DaemonInfo};

async fn handle_download_file(info: DaemonInfo, query: Query) -> eyre::Result<()> {
    // path should be something like "{dataflow_name}/{file_name}"
    let path = query
        .attachment()
        .ok_or_else(|| eyre::eyre!("Query doesn't contain any attachment"))?
        .try_to_string()?
        .into_owned();

    let home =
        simple_home_dir::home_dir().ok_or_else(|| eyre::eyre!("Couldn't find home directory"))?;

    let path = home.join(".narr").join(path);

    let bytes = query
        .payload()
        .ok_or_else(|| eyre::eyre!("Query doesn't contain any payload"))?
        .to_bytes();

    tokio::fs::create_dir_all(
        path.parent()
            .ok_or_eyre(eyre::eyre!("Couldn't get parent"))?,
    )
    .await?;

    let mut file = tokio::fs::File::create(path).await?;

    file.write_all(&bytes).await?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let metadata = file.metadata().await?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);

        file.set_permissions(permissions).await?;
    }

    query
        .reply(
            format!("narr/daemon/{}/file", info.id),
            DaemonReply::FileSent.to_bytes()?,
        )
        .await
        .map_err(eyre::Report::msg)
}

pub async fn spawn(
    session: Arc<Session>,
    info: DaemonInfo,
    mut abort_rx: Receiver<()>,
) -> eyre::Result<JoinHandle<()>> {
    let download_file = session
        .declare_queryable(format!("narr/daemon/{}/file", info.id))
        .await
        .map_err(eyre::Report::msg)?;

    Ok(tokio::task::spawn(async move {
        loop {
            tokio::select! {
                query = download_file.recv_async() => {
                    if let Ok(query) = query {
                        if let Err(e) = handle_download_file(info.clone(), query).await {
                            tracing::error!("Error handling query: {}", e);
                        }
                    } else if let Err(error) = query {
                        tracing::error!("Error receiving query: {}", error);
                    }
                }
                _ = abort_rx.recv() => {
                    tracing::info!("Received abort signal");
                    break;
                }
            }
        }
    }))
}
