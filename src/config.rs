use crate::semver::Level as SemVerLevel;
use anyhow::Result;
use config::{Config, Environment};
use log::Level as LogLevel;

pub struct MinverConfig {
    pub log_level: LogLevel,
    pub auto_increment_level: SemVerLevel,
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
            auto_increment_level: settings
                .get_str("auto_increment_level")
                .unwrap_or(String::from("Patch"))
                .parse()?,
        })
    }
}
