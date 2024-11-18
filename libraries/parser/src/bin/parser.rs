use narr_parser::ApplicationConfig;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let yaml_content = tokio::fs::read_to_string("examples/talker-listener/dataflow.yml").await?;

    let config = ApplicationConfig::parse_yaml(&yaml_content)?;

    println!("Parsed Configuration: {:?}", config);

    // Convert to Application struct if needed
    let application = config.to_application()?;
    println!("Converted Application: {:?}", application);

    Ok(())
}
