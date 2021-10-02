use crate::semver::Level as SemVerLevel;
use anyhow::Result;
use config::{Config, ConfigError, Environment};
use log::Level as LogLevel;
use regex::Regex;

const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Warn;
const DEFAULT_INCREMENT_LEVEL: SemVerLevel = SemVerLevel::Patch;
const DEFAULT_BUILD_METADATA: Option<String> = None;

pub struct MinverConfig {
    pub log_level: LogLevel,
    pub auto_increment_level: SemVerLevel,
    pub build_metadata: Option<String>,
}

impl MinverConfig {
    pub fn read_from_env() -> Result<MinverConfig> {
        let mut settings = Config::default();
        settings.merge(Environment::with_prefix("MINVER"))?;

        Ok(MinverConfig {
            log_level: settings
                .get_str("log_level")
                .unwrap_or(DEFAULT_LOG_LEVEL.to_string())
                .parse()?,
            auto_increment_level: settings
                .get_str("auto_increment_level")
                .unwrap_or(DEFAULT_INCREMENT_LEVEL.to_string())
                .parse()?,
            build_metadata: settings
                .get_str("build_metadata")
                .and_then(|str| check_build_metadata(str))
                .map(|str| Some(str))
                .unwrap_or(DEFAULT_BUILD_METADATA),
        })
    }

    pub fn default() -> MinverConfig {
        MinverConfig {
            log_level: DEFAULT_LOG_LEVEL,
            auto_increment_level: DEFAULT_INCREMENT_LEVEL,
            build_metadata: DEFAULT_BUILD_METADATA,
        }
    }
}

fn check_build_metadata(metadata: String) -> Result<String, ConfigError> {
    // Regex partially taken from https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
    let pattern = "[0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*";

    let re = Regex::new(pattern).unwrap();
    if re.is_match(&metadata) {
        Ok(metadata)
    } else {
        Err(ConfigError::Message(format!(
            "{} is not valid build metadata",
            metadata
        )))
    }
}
