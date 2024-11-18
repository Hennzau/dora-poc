use narr_rs::prelude::{DaemonAddress, DaemonQuery, DaemonReply};

use crate::create_cli_session;

pub async fn daemon_check(id: String, connect: DaemonAddress) -> eyre::Result<()> {
    let session = create_cli_session(connect).await?;

    let query = session
        .get(format!("narr/daemon/{}/query", id))
        .payload(DaemonQuery::Check.to_bytes()?)
        .await
        .map_err(eyre::Report::msg)?;

    while let Ok(reply) = query.recv_async().await {
        match reply.result() {
            Ok(reply) => {
                if let Ok(reply) = DaemonReply::from_bytes(&reply.payload().to_bytes().into_owned())
                {
                    if let DaemonReply::Ok(info) = reply {
                        println!("{}: OK", info.id);
                    } else {
                        tracing::error!("Received unexpected reply: {:?}", reply);
                    }
                } else {
                    tracing::error!("Received invalid reply: {:?}", reply);
                }
            }
            Err(err) => {
                tracing::error!("Error receiving reply: {:?}", err);
            }
        }
    }

    Ok(())
}
