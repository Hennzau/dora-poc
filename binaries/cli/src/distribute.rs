use eyre::OptionExt;
use narr_rs::prelude::{Application, DaemonQuery, DaemonReply};

use crate::{create_cli_session, validate::daemon_validate};

pub async fn daemon_distribute(application: Application) -> eyre::Result<()> {
    let connect = application.network.values().cloned().collect();

    let session = create_cli_session(connect).await?;

    daemon_validate(application.clone()).await?;

    for (node_id, node) in application.nodes {
        let receiver = application
            .distribution
            .get(&node_id)
            .ok_or_eyre("Receiver not found")?;

        for (sender, path) in node.files {
            let file_name = path
                .file_name()
                .ok_or_eyre("Invalid file path")?
                .to_str()
                .ok_or_eyre("Invalid file name")?;

            let new_name = format!("{}/{}/{}", application.id, node_id, file_name);

            let query = session
                .get(format!("narr/daemon/{}/query", sender))
                .payload(DaemonQuery::SendFile(receiver.clone(), path, new_name).to_bytes()?)
                .await
                .map_err(eyre::Report::msg)?;

            let reply = query.recv_async().await.map_err(eyre::Report::msg)?;

            match reply.result() {
                Ok(reply) => {
                    if let Ok(reply) = DaemonReply::from_bytes(&reply.payload().to_bytes()) {
                        tracing::info!("Received reply: {:?}", reply);
                    } else {
                        tracing::error!("Received invalid reply: {:?}", reply);
                    }
                }
                Err(err) => {
                    tracing::error!("Error receiving reply: {:?}", err);
                }
            }
        }
    }

    Ok(())
}
