use std::path::PathBuf;

use zenoh::query::Query;

use crate::{queries::DaemonReply, DaemonInfo};

pub async fn handle_check(info: DaemonInfo, query: Query) -> eyre::Result<()> {
    let listen = info
        .listen
        .iter()
        .map(|address| address.to_string())
        .collect::<Vec<String>>();

    let reachable = format!("{:?}", listen);
    let id = info.id.clone();

    query
        .reply(
            format!("narr/daemon/{}/query", info.id),
            DaemonReply::Ok(crate::queries::InfoReply { id, reachable }).to_bytes()?,
        )
        .await
        .map_err(eyre::Report::msg)?;

    Ok(())
}

pub async fn handle_check_file(info: DaemonInfo, file: PathBuf, query: Query) -> eyre::Result<()> {
    let reply = match file.exists() {
        true => DaemonReply::FileOk,
        false => DaemonReply::FileNotFound,
    };

    query
        .reply(format!("narr/daemon/{}/query", info.id), reply.to_bytes()?)
        .await
        .map_err(eyre::Report::msg)?;

    Ok(())
}
