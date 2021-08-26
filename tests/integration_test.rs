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

#[test]
fn test_height_is_appended_to_version() {
    let dir = TempDir::new().unwrap();
    let repo = repo_test_helper::create_temp_repo(dir.path()).unwrap();

    repo_test_helper::commit_on_head(&repo).unwrap();
    repo_test_helper::tag_head(&repo, "1.2.3").unwrap();
    repo_test_helper::commit_on_head(&repo).unwrap();

    assert_eq!(
        Version { major: 1, minor: 2, patch: 3, prerelease: Some(String::from("alpha.1")), build_metadata: None },
        minver_rs::get_version(&repo).unwrap());
}

#[test]
#[ignore]
fn test_when_no_tags_are_present_in_ancestors_then_default_version_is_returned() {
    assert!(true, "Not implemented")
}

#[test]
#[ignore]
fn test_when_lower_tag_is_more_recent_then_older_version_is_returned() {
    assert!(true, "Not implemented")
}

#[test]
#[ignore]
fn test_when_old_commit_is_checked_out_then_newer_tags_are_ignored() {
    assert!(true, "Not implemented")
}

#[test]
#[ignore]
fn test_when_branches_diverge_then_higher_tag_is_used() {
    assert!(true, "Not implemented")
}

#[test]
#[ignore]
fn test_when_branches_merge_then_lower_height_is_used() {
    assert!(true, "Not implemented")
}

#[test]
#[ignore]
fn test_when_merged_branch_has_lower_version_tag_then_main_branch_version_is_returned() {
    assert!(true, "Not implemented")
}

#[test]
#[ignore]
fn test_when_build_metadata_is_present_in_tagged_head_then_metadata_is_included_in_version() {
    assert!(true, "Not implemented")
}

#[test]
#[ignore]
fn test_when_build_metadata_is_present_in_old_tag_then_metadata_is_ignored() {
    assert!(true, "Not implemented")
}