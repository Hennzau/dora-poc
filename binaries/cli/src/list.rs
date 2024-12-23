use comfy_table::Table;
use dpoc_rs::prelude::{DaemonAddress, DaemonQuery, DaemonReply};

use crate::create_cli_session;

pub async fn daemon_list(connect: DaemonAddress) -> eyre::Result<()> {
    let session = create_cli_session(vec![connect]).await?;

    let query = session
        .get("dpoc/daemon/*/query")
        .payload(DaemonQuery::Check.to_bytes()?)
        .await
        .map_err(eyre::Report::msg)?;

    let mut table = Table::new();
    table.set_width(80);
    table.set_header(vec!["Daemon ID", "Reachable"]);

    while let Ok(reply) = query.recv_async().await {
        match reply.result() {
            Ok(reply) => {
                if let Ok(reply) = DaemonReply::from_bytes(&reply.payload().to_bytes()) {
                    if let DaemonReply::Ok(info) = reply {
                        table.add_row(vec![info.id, info.reachable]);
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

    println!("{}", table);

    Ok(())
}
