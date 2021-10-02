use std::collections::HashSet;

use anyhow::Result;
use git2::{Commit, Oid, Repository};
use radix_trie::Trie;

pub use crate::semver::Version;
use crate::MinverConfig;

pub fn get_version(repository: &Repository, config: &MinverConfig) -> Result<Version> {
    log::debug!("Getting version for {:?}", repository.path());
    let tags = get_tags(repository)?;

    let (version, height) = find_latest_versions(&tags, &repository)?
        .into_iter()
        .max_by(|(v1, _h1), (v2, _h2)| v1.cmp_precedence(v2))
        .unwrap_or_else(|| {
            let v = Version::default();
            log::debug!("No tags found, using {}", v);
            (v, 0)
        });

    if height == 0 {
        log::debug!("Height is zero, leaving tag as-is: {}", version);
        Ok(version)
    } else {
        log::debug!(
            "Height is non-zero, removing metadata and incrementing version from {}",
            version
        );
        Ok(version
            .with_height(height)
            .without_metadata()
            .with_incremented_level(&config.auto_increment_level))
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

fn get_tags(repository: &Repository) -> Result<Trie<String, Version>> {
    // Note: A trie may or may not actually be more performant than a map, but I'm using it anyways
    // because it's theoretically more efficient and I don't get to use tries very often :)
    let mut trie = Trie::new();

    // TODO: Use pattern to filter tags
    let tags = repository.tag_names(None)?;
    tags.iter()
        .filter_map(|opt| {
            if opt.is_none() {
                log::debug!("Found non UTF-8 tag, ignoring it");
            }
            opt
        })
        .map(|tag_name| {
            Ok((
                Version::parse(tag_name)?,
                get_tagged_commit(&repository, tag_name)?,
            ))
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
