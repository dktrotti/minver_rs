//! minver_rs is a minimal version management tool.
//! 
//! Automatically sets the crate version based on git tags. If the current commit is not tagged
//! with a version, the version will be set to a prerelease version with the height from the
//! latest tag appended to the version.
//! 
//! To use this crate, call [`build_utils::default_build_action()`] in `build.rs`, then set
//! [`build_utils::UPDATE_VERSION_VAR`].
//! 
//! See [`MinverConfig`] for details on available options.
//! 
//! Based on https://github.com/adamralph/minver

pub mod build_utils;
mod config;
mod minver_core;
mod semver;

pub use crate::config::MinverConfig;
pub use minver_core::get_version;
pub use minver_core::Version;
pub use semver::Level as SemVerLevel;
