use git2::{Repository, Signature};
use tempfile::TempDir;

use minver_rs::*;

#[test]
fn test_tagged_head_returns_tag_version() {
    let dir = TempDir::new().unwrap();
    let repo = Repository::init(dir.path()).unwrap();

    let signature = Signature::now("testName", "test@example.com").unwrap();
    let mut index = repo.index().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    let commit_id_1 = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "test message",
        &tree,
        &[]
    ).unwrap();
    let commit_1 = repo.find_commit(commit_id_1).unwrap();

    let commit_id_2 = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "test message 2",
        &tree,
        &[&commit_1]
    ).unwrap();
    let commit_2 = repo.find_commit(commit_id_2).unwrap();

    repo.tag(
        "1.2.3",
        &commit_2.as_object(),
        &signature,
        "created tag",
        false
    ).unwrap();

    // TODO: Check version
    assert_eq!(
        Version { major: 1, minor: 2, patch: 3, prerelease: None, build_metadata: None },
        minver_rs::get_version(&repo).unwrap());
}