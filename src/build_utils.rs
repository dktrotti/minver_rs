use anyhow::Result;
use git2::Repository;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use toml_edit::{value, Document};

use crate::MinverConfig;

/// Environment variable that determines if minver_rs will update the crate version. If this
/// variable is not set, then no action will be taken.
pub const UPDATE_VERSION_VAR: &str = "MINVER_UPDATE_VERSION";

/// Updates the version in `Cargo.toml`.
/// 
/// This function prints `cargo:rerun-if` output to ensure that this build action will be run when
/// called in `build.rs`.
/// 
/// ```no_run
/// use minver_rs::build_utils;
/// 
/// fn main() {
///     if env!("CARGO_PKG_NAME") != env!("CARGO_CRATE_NAME") {
///         build_utils::default_build_action();
///     }
/// }
/// ```
pub fn default_build_action() {
    println!("cargo:rerun-if-changed=.git/refs/tags/");
    println!("cargo:rerun-if-env-changed={}", UPDATE_VERSION_VAR);

    let config = MinverConfig::read_from_env().expect("Failed to parse configuration");
    if let Err(e) = simple_logger::init_with_level(config.log_level) {
        println!("Failed to initialize log: {}", e);
    }

    default_build_action_silent(&config);
}

/// Updates the version in `Cargo.toml` without printing any `cargo:rerun-if` output.
pub fn default_build_action_silent(config: &MinverConfig) {
    if env::var_os(UPDATE_VERSION_VAR).is_some() {
        update_package_version(
            &env::var_os("CARGO_MANIFEST_DIR").unwrap_or(OsString::from(".")),
            config,
        )
        .unwrap()
    } else {
        log::info!(
            "Environment variable {} is not set, no action will be taken",
            UPDATE_VERSION_VAR
        );
    }
}

/// Updates the version in `Cargo.toml`.
pub fn update_package_version(manifest_dir: &OsString, config: &MinverConfig) -> Result<()> {
    let manifest_path = Path::new(manifest_dir).join("Cargo.toml");
    log::debug!("Will update manifest file at {:?}", manifest_path);

    let mut document: Document = fs::read_to_string(&manifest_path)?.parse::<Document>()?;
    log::debug!("Successfully read manifest file");

    match Repository::open(manifest_dir) {
        Ok(repo) => {
            let version = crate::get_version(&repo, config)?;

            document["package"]["version"] = value(version.to_string());
            log::debug!("Updated version to {}", version);

            Ok(fs::write(
                &manifest_path,
                document.to_string_in_original_order(),
            )?)
        }
        Err(_) => {
            // If we're not being built from our repo, the version doesn't need to be set
            log::info!("Build util run outside of repository, manifest file will not be updated");
            Ok(())
        }
    }
}
