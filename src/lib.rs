mod semver;

use git2::{Repository, Commit};
use anyhow::Result;
use radix_trie::Trie;

pub use semver::Version;

pub fn get_version(repository: &Repository) -> Result<Version> {
    let tags = get_tags(repository)?;

    let head_commit = repository.head()?.peel_to_commit()?;

    let (version, height) = find_latest_versions(&tags, &head_commit, 0)?
        .into_iter()
        .max_by(|(v1, _h1), (v2, _h2)| {
            v1.cmp_precedence(v2)
        })
        .unwrap_or((Version::default(), 0));
    
    if height == 0 {
        Ok(version)
    } else {
        Ok(version.with_height(height))
    }
}

// TODO: Should recursion be used given how large a git graph can be?
fn find_latest_versions(tags: &Trie<String, Version>, commit: &Commit, height: u32) -> Result<Vec<(Version, u32)>> {
    match tags.get(&commit.id().to_string()) {
        Some(v) => Ok(vec!((v.clone(), height))),
        None => {
            // TODO: Handle multiple parents
            // (but also account for case where there are multiple merged branches without any tags)
            if commit.parent_count() == 0 {
                Ok(vec!())
            } else {
                let parent_versions: Result<Vec<Vec<(Version, u32)>>> = commit.parents()
                    .map(|parent| {
                        find_latest_versions(tags, &parent, height + 1)
                    })
                    .collect::<Result<Vec<Vec<(Version, u32)>>>>();
            
                parent_versions.map(|vec| {
                    vec.into_iter().flatten().collect()
                })
            }

        }
    }
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
            Ok((Version::parse(tag_name)?, get_tagged_commit(&repository, tag_name)?))
        })
        // TODO: Log version parse failures
        .filter_map(|result: Result<(Version, Commit)>| {
            result.ok()
        })
        .for_each(|(version, commit)| {
            trie.insert(commit.id().to_string(), version);
        });

    Ok(trie)
}

fn get_tagged_commit<'a>(repository: &'a Repository, tag_name: &'a str) -> Result<Commit<'a>> {
    let object = repository.revparse_single(&format!("refs/tags/{}", tag_name))?;
    Ok(object.peel_to_commit()?)
}