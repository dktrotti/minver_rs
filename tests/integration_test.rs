use tempfile::TempDir;

use minver_rs::*;

mod repo_test_helper;

#[test]
fn test_tagged_head_returns_tag_version() {
    let dir = TempDir::new().unwrap();
    let repo = repo_test_helper::create_temp_repo(dir.path()).unwrap();

    repo_test_helper::commit_on_head(&repo, "m").unwrap();
    repo_test_helper::tag_head(&repo, "1.2.3").unwrap();

    assert_eq!(
        Version {
            major: 1,
            minor: 2,
            patch: 3,
            prerelease: None,
            build_metadata: None
        },
        minver_rs::get_version(&repo).unwrap()
    );
}

#[test]
fn test_height_is_appended_to_version() {
    let dir = TempDir::new().unwrap();
    let repo = repo_test_helper::create_temp_repo(dir.path()).unwrap();

    repo_test_helper::commit_on_head(&repo, "m1").unwrap();
    repo_test_helper::tag_head(&repo, "1.2.3").unwrap();
    repo_test_helper::commit_on_head(&repo, "m2").unwrap();

    assert_eq!(
        Version {
            major: 1,
            minor: 2,
            patch: 3,
            prerelease: Some(String::from("alpha.1")),
            build_metadata: None
        },
        minver_rs::get_version(&repo).unwrap()
    );
}

#[test]
fn test_when_no_tags_are_present_in_ancestors_then_default_version_is_returned() {
    let dir = TempDir::new().unwrap();
    let repo = repo_test_helper::create_temp_repo(dir.path()).unwrap();

    repo_test_helper::commit_on_head(&repo, "m1").unwrap();
    repo_test_helper::commit_on_head(&repo, "m2").unwrap();

    assert_eq!(
        Version {
            major: 0,
            minor: 0,
            patch: 0,
            prerelease: Some(String::from("alpha.0")),
            build_metadata: None
        },
        minver_rs::get_version(&repo).unwrap()
    );
}

#[test]
fn test_when_repo_access_fails_then_error_is_returned() {
    let dir = TempDir::new().unwrap();
    let repo = repo_test_helper::create_temp_repo(dir.path()).unwrap();

    repo_test_helper::commit_on_head(&repo, "m1").unwrap();
    repo_test_helper::commit_on_head(&repo, "m2").unwrap();

    dir.close().unwrap();

    let err = minver_rs::get_version(&repo).err();
    assert!(err.is_some());
}

#[test]
fn test_when_lower_tag_is_more_recent_then_older_version_is_returned() {
    let dir = TempDir::new().unwrap();
    let repo = repo_test_helper::create_temp_repo(dir.path()).unwrap();

    repo_test_helper::commit_on_head(&repo, "m1").unwrap();
    repo_test_helper::tag_head(&repo, "2.0.0").unwrap();
    repo_test_helper::commit_on_head(&repo, "m2").unwrap();
    repo_test_helper::tag_head(&repo, "1.2.3").unwrap();

    assert_eq!(
        Version {
            major: 1,
            minor: 2,
            patch: 3,
            prerelease: None,
            build_metadata: None
        },
        minver_rs::get_version(&repo).unwrap()
    );
}

#[test]
fn test_when_old_commit_is_checked_out_then_newer_tags_are_ignored() {
    let dir = TempDir::new().unwrap();
    let repo = repo_test_helper::create_temp_repo(dir.path()).unwrap();

    repo_test_helper::commit_on_head(&repo, "m1").unwrap();
    repo_test_helper::tag_head(&repo, "1.2.3").unwrap();
    let intermediate_commit = repo_test_helper::commit_on_head(&repo, "m2").unwrap();
    repo_test_helper::commit_on_head(&repo, "m3").unwrap();
    repo_test_helper::tag_head(&repo, "1.3.0").unwrap();

    repo_test_helper::checkout_commit(&repo, &intermediate_commit).unwrap();

    assert_eq!(
        Version {
            major: 1,
            minor: 2,
            patch: 3,
            prerelease: Some(String::from("alpha.1")),
            build_metadata: None
        },
        minver_rs::get_version(&repo).unwrap()
    );
}

#[test]
fn test_when_branches_diverge_with_multiple_tags_then_higher_tag_is_used() {
    let dir = TempDir::new().unwrap();
    let repo = repo_test_helper::create_temp_repo(dir.path()).unwrap();

    let commit_1 = repo_test_helper::commit_on_head(&repo, "c1").unwrap();

    let branch_1_commit_1 = repo_test_helper::commit_with_parent(&repo, &commit_1, "b1c1").unwrap();
    repo_test_helper::tag_commit(&repo, &branch_1_commit_1, "1.3.0").unwrap();
    let branch_1_commit_2 =
        repo_test_helper::commit_with_parent(&repo, &branch_1_commit_1, "b1c2").unwrap();

    repo_test_helper::checkout_commit(&repo, &commit_1).unwrap();
    let branch_2_commit_1 = repo_test_helper::commit_with_parent(&repo, &commit_1, "b2c1").unwrap();
    repo_test_helper::tag_commit(&repo, &branch_2_commit_1, "1.2.3").unwrap();

    repo_test_helper::merge_commit(&repo, &[&branch_2_commit_1, &branch_1_commit_2], "m").unwrap();

    assert_eq!(
        Version {
            major: 1,
            minor: 3,
            patch: 0,
            prerelease: Some(String::from("alpha.2")),
            build_metadata: None
        },
        minver_rs::get_version(&repo).unwrap()
    );
}

#[test]
fn test_when_branches_merge_with_same_tagged_parent_then_lower_height_is_used() {
    let dir = TempDir::new().unwrap();
    let repo = repo_test_helper::create_temp_repo(dir.path()).unwrap();

    let commit_1 = repo_test_helper::commit_on_head(&repo, "c1").unwrap();
    repo_test_helper::tag_commit(&repo, &commit_1, "1.2.3").unwrap();

    let branch_1_commit_1 = repo_test_helper::commit_with_parent(&repo, &commit_1, "b1c1").unwrap();
    let branch_1_commit_2 =
        repo_test_helper::commit_with_parent(&repo, &branch_1_commit_1, "b1c2").unwrap();

    repo_test_helper::checkout_commit(&repo, &commit_1).unwrap();
    let branch_2_commit_1 = repo_test_helper::commit_with_parent(&repo, &commit_1, "b2c1").unwrap();

    repo_test_helper::checkout_commit(&repo, &branch_1_commit_2).unwrap();
    repo_test_helper::merge_commit(&repo, &[&branch_1_commit_2, &branch_2_commit_1], "m").unwrap();

    assert_eq!(
        Version {
            major: 1,
            minor: 2,
            patch: 3,
            prerelease: Some(String::from("alpha.2")),
            build_metadata: None
        },
        minver_rs::get_version(&repo).unwrap()
    );
}

#[test]
fn test_when_merged_branch_has_lower_version_tag_then_main_branch_version_is_returned() {
    let dir = TempDir::new().unwrap();
    let repo = repo_test_helper::create_temp_repo(dir.path()).unwrap();

    let commit_1 = repo_test_helper::commit_on_head(&repo, "c1").unwrap();
    repo_test_helper::tag_commit(&repo, &commit_1, "1.3.0").unwrap();

    let branch_1_commit_1 = repo_test_helper::commit_with_parent(&repo, &commit_1, "b1c1").unwrap();
    let branch_1_commit_2 =
        repo_test_helper::commit_with_parent(&repo, &branch_1_commit_1, "b1c2").unwrap();

    repo_test_helper::checkout_commit(&repo, &commit_1).unwrap();
    let branch_2_commit_1 = repo_test_helper::commit_with_parent(&repo, &commit_1, "b2c1").unwrap();
    repo_test_helper::tag_commit(&repo, &commit_1, "1.2.3").unwrap();

    repo_test_helper::checkout_commit(&repo, &branch_1_commit_2).unwrap();
    repo_test_helper::merge_commit(&repo, &[&branch_1_commit_2, &branch_2_commit_1], "m").unwrap();

    assert_eq!(
        Version {
            major: 1,
            minor: 3,
            patch: 0,
            prerelease: Some(String::from("alpha.2")),
            build_metadata: None
        },
        minver_rs::get_version(&repo).unwrap()
    );
}

#[test]
fn test_when_build_metadata_is_present_in_tagged_head_then_metadata_is_included_in_version() {
    let dir = TempDir::new().unwrap();
    let repo = repo_test_helper::create_temp_repo(dir.path()).unwrap();

    repo_test_helper::commit_on_head(&repo, "m").unwrap();
    repo_test_helper::tag_head(&repo, "1.2.3+a1b2c3").unwrap();

    assert_eq!(
        Version {
            major: 1,
            minor: 2,
            patch: 3,
            prerelease: None,
            build_metadata: Some(String::from("a1b2c3"))
        },
        minver_rs::get_version(&repo).unwrap()
    );
}

#[test]
fn test_when_build_metadata_is_present_in_old_tag_then_metadata_is_ignored() {
    let dir = TempDir::new().unwrap();
    let repo = repo_test_helper::create_temp_repo(dir.path()).unwrap();

    repo_test_helper::commit_on_head(&repo, "m1").unwrap();
    repo_test_helper::tag_head(&repo, "1.2.3+a1b2c3").unwrap();
    repo_test_helper::commit_on_head(&repo, "m2").unwrap();

    assert_eq!(
        Version {
            major: 1,
            minor: 2,
            patch: 3,
            prerelease: Some(String::from("alpha.1")),
            build_metadata: None
        },
        minver_rs::get_version(&repo).unwrap()
    );
}
