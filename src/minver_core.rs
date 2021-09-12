use std::collections::HashSet;

use anyhow::Result;
use git2::{Commit, Oid, Repository};
use radix_trie::Trie;

use crate::semver::Level;
pub use crate::semver::Version;

pub fn get_version(repository: &Repository) -> Result<Version> {
    let tags = get_tags(repository)?;

    let (version, height) = find_latest_versions(&tags, &repository)?
        .into_iter()
        .max_by(|(v1, _h1), (v2, _h2)| v1.cmp_precedence(v2))
        .unwrap_or((Version::default(), 0));

    if height == 0 {
        Ok(version)
    } else {
        Ok(version
            .with_height(height)
            .without_metadata()
            .with_incremented_level(Level::Patch))
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
            if checked_commits.contains(&commit.id()) {
                continue;
            }
            checked_commits.insert(commit.id());

            // This could be optimized further by using Trie::remove rather than Trie::get to avoid
            // calling Version::clone
            match tags.get(&commit.id().to_string()) {
                Some(v) => results.push((v.clone(), current_height)),
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

    let tags = repository.tag_names(None)?;
    tags.iter()
        // TODO: Non UTF-8 tags are ignored, should they be handled?
        .filter_map(std::convert::identity)
        .map(|tag_name| {
            Ok((
                Version::parse(tag_name)?,
                get_tagged_commit(&repository, tag_name)?,
            ))
        })
        // TODO: Log version parse failures
        .filter_map(|result: Result<(Version, Commit)>| result.ok())
        .for_each(|(version, commit)| {
            trie.insert(commit.id().to_string(), version);
        });

    Ok(trie)
}

fn get_tagged_commit<'a>(repository: &'a Repository, tag_name: &'a str) -> Result<Commit<'a>> {
    let object = repository.revparse_single(&format!("refs/tags/{}", tag_name))?;
    Ok(object.peel_to_commit()?)
}
