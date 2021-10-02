use crate::semver::Level as SemVerLevel;
use anyhow::Result;
use config::{Config, ConfigError, Environment};
use log::Level as LogLevel;
use regex::Regex;

const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Warn;
const DEFAULT_INCREMENT_LEVEL: SemVerLevel = SemVerLevel::Patch;
const DEFAULT_BUILD_METADATA: Option<String> = None;
const DEFAULT_PRERELEASE_IDENTIFIER: &str = "alpha";
const DEFAULT_TAG_PREFIX: &str = "";

#[derive(Debug)]
pub struct MinverConfig {
    pub log_level: LogLevel,
    pub auto_increment_level: SemVerLevel,
    pub build_metadata: Option<String>,
    pub prerelease_identifier: String,
    pub tag_prefix: String,
}

impl MinverConfig {
    pub fn read_from_env() -> Result<MinverConfig> {
        let mut settings = Config::default();
        settings.merge(Environment::with_prefix("MINVER"))?;

        Ok(MinverConfig {
            log_level: match settings.get_str("log_level") {
                Ok(str) => str.parse()?,
                Err(_) => DEFAULT_LOG_LEVEL,
            },
            auto_increment_level: match settings.get_str("auto_increment_level") {
                Ok(str) => str.parse()?,
                Err(_) => DEFAULT_INCREMENT_LEVEL,
            },
            build_metadata: match settings.get_str("build_metadata") {
                Ok(str) => {
                    check_build_metadata(&str)?;
                    Some(str)
                }
                Err(_) => DEFAULT_BUILD_METADATA,
            },
            prerelease_identifier: match settings.get_str("prerelease_identifier") {
                Ok(str) => {
                    check_prerelease_identifier(&str)?;
                    str
                }
                Err(_) => String::from(DEFAULT_PRERELEASE_IDENTIFIER),
            },
            tag_prefix: settings
                .get_str("tag_prefix")
                .unwrap_or(String::from(DEFAULT_TAG_PREFIX)),
        })
    }

    pub fn default() -> MinverConfig {
        MinverConfig {
            log_level: DEFAULT_LOG_LEVEL,
            auto_increment_level: DEFAULT_INCREMENT_LEVEL,
            build_metadata: DEFAULT_BUILD_METADATA,
            prerelease_identifier: String::from(DEFAULT_PRERELEASE_IDENTIFIER),
            tag_prefix: String::from(DEFAULT_TAG_PREFIX),
        }
    }
}

fn check_build_metadata(metadata: &String) -> Result<(), ConfigError> {
    // Regex partially taken from https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
    let pattern = "^[0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*$";

    let re = Regex::new(pattern).unwrap();
    if re.is_match(&metadata) {
        Ok(())
    } else {
        Err(ConfigError::Message(format!(
            "{} is not valid build metadata",
            metadata
        )))
    }
}

fn check_prerelease_identifier(identifier: &String) -> Result<(), ConfigError> {
    // Regex partially taken from https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
    let pattern = "^(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*$";

    let re = Regex::new(pattern).unwrap();
    if re.is_match(&identifier) {
        Ok(())
    } else {
        Err(ConfigError::Message(format!(
            "{} is not a valid prerelease identifier",
            identifier
        )))
    }
}
