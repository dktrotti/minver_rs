use anyhow::Result;
use git2::Repository;
use minver_rs;
use std::env;
use std::fs;
use toml_edit::{value, Document};

const MANIFEST_PATH: &str = "./Cargo.toml";
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
    let mut document: Document = fs::read_to_string(MANIFEST_PATH)?.parse::<Document>()?;

    let repo = Repository::open(".")?;
    let version = minver_rs::get_version(&repo)?;

    document["package"]["version"] = value(version.to_string());

    Ok(fs::write(
        MANIFEST_PATH,
        document.to_string_in_original_order(),
    )?)
}
