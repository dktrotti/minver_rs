mod semver;

use git2::Repository;
use anyhow::{Result, anyhow};

pub use semver::Version;

pub fn get_version(repository: &Repository) -> Result<Version> {
    Err(anyhow!("Not implemented"))
}