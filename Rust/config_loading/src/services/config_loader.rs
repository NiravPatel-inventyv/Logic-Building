use config::{Config, ConfigError, Environment, File};
use std::env;

pub fn config_loader() -> Result<Config, ConfigError> {
    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
    let s = Config::builder()
        .add_source(File::with_name("resources/config/config"))
        .add_source(
            File::with_name(&format!("resources/config/config-{}", run_mode)).required(false))
        .add_source(Environment::with_prefix("app"))
        .build()?;
    Ok(s)
}
