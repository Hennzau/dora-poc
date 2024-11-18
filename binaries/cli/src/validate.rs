use narr_rs::prelude::{Application, DaemonAddress};

pub async fn daemon_validate(application: Application, connect: DaemonAddress) -> eyre::Result<()> {
    println!("{:?}", application);

    Ok(())
}
