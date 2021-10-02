pub mod build_utils;
mod config;
mod minver_core;
mod semver;

pub use crate::config::MinverConfig;
pub use minver_core::get_version;
pub use minver_core::Version;
pub use semver::Level as SemVerLevel;
