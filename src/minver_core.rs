use std::collections::HashSet;

use anyhow::Result;
use git2::{Commit, Oid, Repository};
use radix_trie::Trie;

pub use crate::semver::Version;
use crate::MinverConfig;

/// Calculates the version for the given repository and configuration.
pub fn get_version(repository: &Repository, config: &MinverConfig) -> Result<Version> {
    log::info!("Getting version for {:?}", repository.path());
    log::debug!("Loaded config: {:?}", repository.path());
    let tags = get_tags(repository, &config.tag_prefix)?;

    let (version, height) = find_latest_versions(&tags, &repository)?
        .into_iter()
        .max_by(|(v1, _h1), (v2, _h2)| v1.cmp_precedence(v2))
        .unwrap_or_else(|| {
            let v = Version::default(&config.prerelease_identifier);
            log::debug!("No tags found, using {}", v);
            (v, 0)
        });

    let version = if height == 0 {
        log::debug!("Height is zero, leaving tag as-is: {}", version);
        version
    } else {
        log::debug!(
            "Height is non-zero, removing metadata and incrementing {} version from {}",
            &config.auto_increment_level,
            version
        );
        version
            .with_height(height, &config.prerelease_identifier)
            .without_metadata()
            .with_incremented_level(&config.auto_increment_level)
    };

    match &config.build_metadata {
        Some(metadata) => {
            log::debug!("Appending configured metadata: {}", metadata);
            Ok(version.with_appended_metadata(&metadata))
        }
        None => Ok(version),
    }
}

fn find_latest_versions(
    tags: &Trie<String, Version>,
    repository: &Repository,
) -> Result<Vec<(Version, u32)>> {
    let mut current_height: u32 = 0;
    let mut results: Vec<(Version, u32)> = vec![];

    let mut checked_commits: HashSet<Oid> = HashSet::new();
    let mut commits_to_check = vec![repository.head()?.peel_to_commit()?];

    while !commits_to_check.is_empty() {
        let mut parent_commits: Vec<Vec<Commit>> = vec![];
        for commit in commits_to_check {
            log::trace!("Checking {:?}", &commit);
            if checked_commits.contains(&commit.id()) {
                log::trace!("Commit has already been checked, skipping: {:?}", &commit);
                continue;
            }
            checked_commits.insert(commit.id());

            // This could be optimized further by using Trie::remove rather than Trie::get to avoid
            // calling Version::clone
            match tags.get(&commit.id().to_string()) {
                Some(v) => {
                    log::trace!("Found candidate version: {} at {:?}", &v, &commit);
                    results.push((v.clone(), current_height))
                }
                None => parent_commits.push(commit.parents().collect()),
            }
        }

        commits_to_check = parent_commits.into_iter().flatten().collect();
        current_height = current_height + 1;
    }

    Ok(results)
}

fn get_tags(repository: &Repository, tag_prefix: &str) -> Result<Trie<String, Version>> {
    // Note: A trie may or may not actually be more performant than a map, but I'm using it anyways
    // because it's theoretically more efficient and I don't get to use tries very often :)
    let mut trie = Trie::new();

    let tags = repository.tag_names(None)?;
    tags.iter()
        .filter_map(|opt| {
            if opt.is_none() {
                log::debug!("Found non UTF-8 tag, ignoring it");
            }
            opt
        })
        .filter_map(|tag_name| {
            let result_opt = tag_name.strip_prefix(tag_prefix).map(|version| {
                Ok((
                    Version::parse(version)?,
                    get_tagged_commit(&repository, tag_name)?,
                ))
            });

            if result_opt.is_none() {
                log::trace!("Ignoring tag that does not match prefix: {}", tag_name);
            }

            result_opt
        })
        .filter_map(|result: Result<(Version, Commit)>| {
            if result.is_err() {
                log::warn!(
                    "Error occurred while handling tag: {}",
                    result.as_ref().err().unwrap()
                )
            }
            result.ok()
        })
        .for_each(|(version, commit)| {
            log::trace!("Found tag {} for {:?}", version, &commit);
            trie.insert(commit.id().to_string(), version);
        });

    Ok(trie)
}

fn get_tagged_commit<'a>(repository: &'a Repository, tag_name: &'a str) -> Result<Commit<'a>> {
    log::trace!("Getting commit for {}", tag_name);
    let object = repository.revparse_single(&format!("refs/tags/{}", tag_name))?;
    let commit = object.peel_to_commit()?;
    log::trace!("Found commit for {}: {:?}", tag_name, &commit);

    Ok(commit)
}
