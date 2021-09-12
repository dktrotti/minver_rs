use anyhow::Result;
use git2::Repository;
use minver_rs;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::{value, Document};

const UPDATE_VERSION_VAR: &str = "MINVER_UPDATE_VERSION";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/refs/tags/");
    println!("cargo:rerun-if-env-changed={}", UPDATE_VERSION_VAR);

    if env::var_os(UPDATE_VERSION_VAR).is_some() {
        set_package_version().unwrap()
    }
}

fn set_package_version() -> Result<()> {
    let manifest_path = get_manifest_path();
    let mut document: Document = fs::read_to_string(&manifest_path)?.parse::<Document>()?;

    let repo = Repository::open(&manifest_path)?;
    let version = minver_rs::get_version(&repo)?;

    document["package"]["version"] = value(version.to_string());

    Ok(fs::write(
        &manifest_path,
        document.to_string_in_original_order(),
    )?)
}

fn get_manifest_path() -> PathBuf {
    let manifest_dir = &env::var_os("CARGO_MANIFEST_DIR").unwrap_or(OsString::from("."));
    Path::new(manifest_dir).join("Cargo.toml")
}
