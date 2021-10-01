use anyhow::Result;
use config::{Config, Environment};
use log::Level;

pub struct MinverConfig {
    pub log_level: Level,
}

impl MinverConfig {
    pub fn read_from_env() -> Result<MinverConfig> {
        let mut settings = Config::default();
        settings.merge(Environment::with_prefix("MINVER"))?;

        Ok(MinverConfig {
            log_level: settings
                .get_str("log_level")
                .unwrap_or(String::from("Warn"))
                .parse()?,
        })
    }
}
