use std::sync::Arc;

use comfy_table::Table;
use narr_rs::prelude::{Daemon, DaemonAddress, DaemonQuery, DaemonReply};
use tokio::signal::ctrl_c;

async fn create_session(connect: DaemonAddress) -> eyre::Result<Arc<zenoh::Session>> {
    let mut zenoh_config = zenoh::Config::default();

    zenoh_config
        .insert_json5(
            "connect/endpoints",
            &serde_json::json!(vec![connect.to_string()]).to_string(),
        )
        .map_err(eyre::Report::msg)?;

    zenoh_config
        .insert_json5(
            "scouting/multicast/enabled",
            &serde_json::json!(false).to_string(),
        )
        .map_err(eyre::Report::msg)?;

    let session = zenoh::open(zenoh_config).await.map_err(eyre::Report::msg)?;

    Ok(Arc::new(session))
}

pub async fn daemon_check(id: String, connect: DaemonAddress) -> eyre::Result<()> {
    let session = create_session(connect).await?;

    let query = session
        .get(format!("narr/daemon/{}/query", id))
        .payload(DaemonQuery::Check.to_bytes()?.as_ref())
        .await
        .map_err(eyre::Report::msg)?;

    let mut table = Table::new();
    table.set_width(80);
    table.set_header(vec!["ID", "Status"]);

    while let Ok(reply) = query.recv_async().await {
        match reply.result() {
            Ok(reply) => {
                if let Ok(reply) = DaemonReply::from_bytes(&reply.payload().to_bytes().into_owned())
                {
                    if let DaemonReply::Ok(id) = reply {
                        table.add_row(vec![id.clone(), "OK".to_string()]);
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

pub async fn daemon_list(connect: DaemonAddress) -> eyre::Result<()> {
    let session = create_session(connect).await?;

    let query = session
        .get("narr/daemon/*/query")
        .payload(DaemonQuery::Check.to_bytes()?.as_ref())
        .await
        .map_err(eyre::Report::msg)?;

    let mut table = Table::new();
    table.set_width(80);
    table.set_header(vec!["ID", "Status"]);

    while let Ok(reply) = query.recv_async().await {
        match reply.result() {
            Ok(reply) => {
                if let Ok(reply) = DaemonReply::from_bytes(&reply.payload().to_bytes().into_owned())
                {
                    if let DaemonReply::Ok(id) = reply {
                        table.add_row(vec![id.clone(), "OK".to_string()]);
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

pub async fn daemon_spawn(
    id: String,
    listen: Vec<DaemonAddress>,
    connect: Vec<DaemonAddress>,
) -> eyre::Result<()> {
    let daemon = Daemon::spawn(id, listen, connect).await?;

    let _ = ctrl_c().await;

    daemon.abort_tx.send(()).await?;

    Ok(())
}
