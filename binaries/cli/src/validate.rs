use std::{collections::HashMap, path::PathBuf, sync::Arc};

use narr_rs::prelude::{Application, DaemonQuery, DaemonReply};
use zenoh::Session;

use crate::create_cli_session;

async fn ensure_daemons(session: Arc<Session>, application: &Application) -> eyre::Result<()> {
    let daemons = application.network.keys().cloned().collect::<Vec<_>>();

    let query = session
        .get("narr/daemon/*/query")
        .payload(DaemonQuery::Check.to_bytes()?)
        .await
        .map_err(eyre::Report::msg)?;

    let mut reachable_daemons = Vec::new();

    while let Ok(reply) = query.recv_async().await {
        match reply.result() {
            Ok(reply) => {
                if let Ok(reply) = DaemonReply::from_bytes(&reply.payload().to_bytes().into_owned())
                {
                    if let DaemonReply::Ok(info) = reply {
                        reachable_daemons.push(info.id);
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

    for daemon in daemons {
        if !reachable_daemons.contains(&daemon) {
            return Err(eyre::eyre!("Daemon {} is not reachable", daemon));
        }
    }

    Ok(())
}

async fn ensure_files(session: Arc<Session>, application: &Application) -> eyre::Result<()> {
    let mut files = HashMap::new();
    for (_, node) in &application.nodes {
        for (daemon, file) in &node.files {
            files.insert(daemon.clone(), PathBuf::from(file));
        }
    }

    for (daemon, file) in files {
        let query = session
            .get(format!("narr/daemon/{}/query", daemon))
            .payload(DaemonQuery::CheckFile(file.clone()).to_bytes()?)
            .await
            .map_err(eyre::Report::msg)?;

        while let Ok(reply) = query.recv_async().await {
            match reply.result() {
                Ok(reply) => {
                    if let Ok(reply) =
                        DaemonReply::from_bytes(&reply.payload().to_bytes().into_owned())
                    {
                        if let DaemonReply::FileNotFound = reply {
                            return Err(eyre::eyre!(
                                "File {:?} not found on daemon {}",
                                file.to_str(),
                                daemon
                            ));
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
    }

    Ok(())
}

async fn ensure_distribution(application: &Application) -> eyre::Result<()> {
    let available_daemons = application.network.keys().cloned().collect::<Vec<_>>();
    let available_nodes = application.nodes.keys().cloned().collect::<Vec<_>>();

    for (node, daemon) in &application.distribution {
        if !available_nodes.contains(node) {
            return Err(eyre::eyre!(
                "Node {} is not defined so it cannot be present in 'distributed' section",
                node
            ));
        }
        if !available_daemons.contains(daemon) {
            return Err(eyre::eyre!(
                "Daemon {} is not defined so it cannot be present in 'distributed' section",
                daemon
            ));
        }
    }

    for node in &available_nodes {
        if !application.distribution.contains_key(node) {
            return Err(eyre::eyre!(
                "Node {} is not parameterized in the 'distributed' section",
                node
            ));
        }
    }

    Ok(())
}

async fn ensure_flows(application: &Application) -> eyre::Result<()> {
    for (input, output) in &application.flows {
        let (input_node, input_id) = input;
        let (output_node, output_id) = output;

        if !application.nodes.contains_key(input_node) {
            return Err(eyre::eyre!(
                "Node {} is not defined so it cannot be present in 'flows' section",
                input_node
            ));
        }

        if !application.nodes.contains_key(output_node) {
            return Err(eyre::eyre!(
                "Node {} is not defined so it cannot be present in 'flows' section",
                output_node
            ));
        }

        if !application.nodes[input_node].inputs.contains(input_id) {
            return Err(eyre::eyre!(
                "Input {} is not defined in node {}",
                input_id,
                input_node
            ));
        }

        if !application.nodes[output_node].outputs.contains(output_id) {
            return Err(eyre::eyre!(
                "Output {} is not defined in node {}",
                output_id,
                output_node
            ));
        }
    }

    Ok(())
}

pub async fn daemon_validate(application: Application) -> eyre::Result<()> {
    let connect = application.network.values().cloned().collect();

    let session = create_cli_session(connect).await?;

    ensure_daemons(session.clone(), &application).await?;
    ensure_files(session.clone(), &application).await?;

    ensure_distribution(&application).await?;
    ensure_flows(&application).await?;

    println!("`{}` is valid", application.id);

    Ok(())
}
