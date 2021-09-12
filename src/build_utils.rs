use anyhow::Result;
use git2::Repository;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use toml_edit::{value, Document};

pub const UPDATE_VERSION_VAR: &str = "MINVER_UPDATE_VERSION";

pub fn default_build_action() {
    println!("cargo:rerun-if-changed=.git/refs/tags/");
    println!("cargo:rerun-if-env-changed={}", UPDATE_VERSION_VAR);

    // Only set the package version if this is the crate being built
    // TODO: Could this be evaluated at compile time to make this function a noop if false?
    if env!("CARGO_PKG_NAME") != env!("CARGO_CRATE_NAME") {
        default_build_action_silent();
    }
}

pub fn default_build_action_silent() {
    if env::var_os(UPDATE_VERSION_VAR).is_some() {
        update_package_version(&env::var_os("CARGO_MANIFEST_DIR").unwrap_or(OsString::from(".")))
            .unwrap()
    }
}

pub fn update_package_version(manifest_dir: &OsString) -> Result<()> {
    let manifest_path = Path::new(manifest_dir).join("Cargo.toml");

    let mut document: Document = fs::read_to_string(&manifest_path)?.parse::<Document>()?;

    match Repository::open(manifest_dir) {
        Ok(repo) => {
            let version = crate::get_version(&repo)?;

            document["package"]["version"] = value(version.to_string());

            Ok(fs::write(
                &manifest_path,
                document.to_string_in_original_order(),
            )?)
        }
        Err(_) => Ok(()), // If we're not being built from our repo, the version doesn't need to be set
    }
}
