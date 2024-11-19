use std::{collections::HashMap, path::PathBuf};

use narr_rs::prelude::{Application, DaemonQuery};
use tokio::io::AsyncWriteExt;

use crate::{create_cli_session, validate::daemon_validate};

pub async fn daemon_distribute(application: Application) -> eyre::Result<()> {
    let connect = application.network.values().cloned().collect();

    let session = create_cli_session(connect).await?;

    let query = session
        .get("narr/daemon/LOCAL_1/query")
        .payload(
            DaemonQuery::File(PathBuf::from(
                "/home/enzo/Documents/narr/target/debug/narr-cli",
            ))
            .to_bytes()?,
        )
        .await
        .map_err(eyre::Report::msg)?;

    while let Ok(reply) = query.recv_async().await {
        match reply.result() {
            Ok(reply) => {
                let bytes = reply.payload().to_bytes();

                let mut file = tokio::fs::File::create("test-cli").await?;

                file.write_all(&bytes).await?;

                // permissions of the file
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;

                    let metadata = file.metadata().await?;
                    let mut permissions = metadata.permissions();
                    permissions.set_mode(0o755);
                    //
                    file.set_permissions(permissions).await?;
                }

                println!("File written");
            }
            Err(err) => {
                tracing::error!("Error receiving reply: {:?}", err);
            }
        }
    }

    // daemon_validate(application.clone()).await?;

    // let mut files = HashMap::new();
    // for node in application.nodes.values() {
    //     for (daemon, file) in &node.files {
    //         files.insert(daemon.clone(), PathBuf::from(file));
    //     }
    // }

    Ok(())
}
