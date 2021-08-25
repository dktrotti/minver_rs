use git2::Repository;
use tempfile::TempDir;

use minver_rs::*;

mod repo_test_helper;

#[test]
fn test_tagged_head_returns_tag_version() {
    let dir = TempDir::new().unwrap();
    let repo = repo_test_helper::create_temp_repo(dir.path()).unwrap();

    repo_test_helper::commit_on_head(&repo).unwrap();
    repo_test_helper::tag_head(&repo, "1.2.3").unwrap();

    assert_eq!(
        Version { major: 1, minor: 2, patch: 3, prerelease: None, build_metadata: None },
        minver_rs::get_version(&repo).unwrap());
}