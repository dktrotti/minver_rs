use anyhow::Result;
use git2::{Commit, Oid, Repository, Signature};
use std::path::Path;

pub fn create_temp_repo(path: &Path) -> Result<Repository> {
    let repo = Repository::init(path).unwrap();

    {
        let mut index = repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        let signature = Signature::now("testName", "test@example.com")?;
        repo.commit(Some("HEAD"), &signature, &signature, "message", &tree, &[])?;
    }

    Ok(repo)
}

pub fn commit_on_head<'a>(repo: &'a Repository, message: &str) -> Result<Commit<'a>> {
    let head_commit = get_head(repo)?;
    commit_with_parent(repo, &head_commit, message)
}

pub fn commit_with_parent<'a>(
    repo: &'a Repository,
    commit: &Commit,
    message: &str,
) -> Result<Commit<'a>> {
    merge_commit(repo, &[&commit], message)
}

pub fn merge_commit<'a>(
    repo: &'a Repository,
    parents: &[&Commit],
    message: &str,
) -> Result<Commit<'a>> {
    let signature = Signature::now("testName", "test@example.com")?;
    let commit_id = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &parents[0].tree()?,
        &parents,
    )?;
    Ok(repo.find_commit(commit_id)?)
}

pub fn tag_head(repo: &Repository, tag: &str) -> Result<Oid> {
    let head_commit = get_head(repo)?;
    tag_commit(repo, &head_commit, tag)
}

pub fn tag_commit(repo: &Repository, commit: &Commit, tag: &str) -> Result<Oid> {
    let signature = Signature::now("testName", "test@example.com")?;
    Ok(repo.tag(tag, commit.as_object(), &signature, "message", false)?)
}

pub fn checkout_commit(repo: &Repository, commit: &Commit) -> Result<()> {
    Ok(repo.set_head_detached(commit.id())?)
}

fn get_head(repo: &Repository) -> Result<Commit> {
    Ok(repo.head()?.peel_to_commit()?)
}
