use anyhow::Result;
use git2::Repository;
use minver_rs;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use toml_edit::{value, Document};

const UPDATE_VERSION_VAR: &str = "MINVER_UPDATE_VERSION";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/refs/tags/");
    println!("cargo:rerun-if-env-changed={}", UPDATE_VERSION_VAR);

    // Only set the package version if this is the crate being built
    if env!("CARGO_PKG_NAME") != env!("CARGO_CRATE_NAME")
        && env::var_os(UPDATE_VERSION_VAR).is_some()
    {
        set_package_version().unwrap()
    }
}

fn set_package_version() -> Result<()> {
    let manifest_dir = &env::var_os("CARGO_MANIFEST_DIR").unwrap_or(OsString::from("."));
    let manifest_path = Path::new(manifest_dir).join("Cargo.toml");

    let mut document: Document = fs::read_to_string(&manifest_path)?.parse::<Document>()?;

    match Repository::open(manifest_dir) {
        Ok(repo) => {
            let version = minver_rs::get_version(&repo)?;

            document["package"]["version"] = value(version.to_string());

            Ok(fs::write(
                &manifest_path,
                document.to_string_in_original_order(),
            )?)
        }
        Err(_) => Ok(()), // If we're not being built from our repo, the version doesn't need to be set
    }
}
