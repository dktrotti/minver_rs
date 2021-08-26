use git2::{Repository, Signature, Commit, Oid};
use anyhow::Result;
use std::path::Path;

pub fn create_temp_repo(path: &Path) -> Result<Repository> {
    let repo = Repository::init(path).unwrap();

    {
        let mut index = repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        let signature = Signature::now("testName", "test@example.com")?;
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "message",
            &tree,
            &[]
        )?;
    }

    Ok(repo)
}

pub fn commit_on_head(repo: &Repository) -> Result<Commit> {
    let head_commit = get_head(repo)?;

    let signature = Signature::now("testName", "test@example.com")?;
    let commit_id = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "message",
        &head_commit.tree()?,
        &[&head_commit] 
    )?;
    Ok(repo.find_commit(commit_id)?)
}

pub fn tag_head(repo: &Repository, tag: &str) -> Result<Oid> {
    let head_commit = get_head(repo)?;

    let signature = Signature::now("testName", "test@example.com")?;
    Ok(repo.tag(
        tag,
        head_commit.as_object(),
        &signature,
        "message",
        false)?)
}

fn get_head(repo: &Repository) -> Result<Commit> {
    Ok(repo.head()?.peel_to_commit()?)
}
