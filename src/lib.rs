mod semver;

use git2::Repository;
use anyhow::{Result, anyhow};

pub use semver::Version;

pub fn get_version(repository: &Repository) -> Result<Version> {
    let tags = repository.tag_names(None)?;

    let versions = tags.iter()
        .filter_map(std::convert::identity)
        .map(|tag_name| {
            Version::parse(tag_name)
        })
        .filter_map(|result| {
            result.ok()
        });
    
    versions.max_by(|v1, v2| { v1.cmp_precedence(v2) })
        .ok_or(anyhow!("No versions found"))
}