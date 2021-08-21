// mod core;
// mod git;
mod semver;

use git2;
use anyhow::Result;

fn main() {
    match get_string(std::path::Path::new("./sampleRepo")) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Error: {}", e)
    }
}

fn get_string(path: &std::path::Path) -> Result<String, git2::Error> {
    let repo = git2::Repository::open_ext(path, git2::RepositoryOpenFlags::NO_SEARCH, std::iter::empty::<String>())?;
    let tags = repo.tag_names(None)?;

    let tagged_commits = tags.iter()
        .filter_map(std::convert::identity)
        .map(|tag_name| {
            Ok((semver::Version::parse(tag_name)?, get_commit(&repo, tag_name)?))
        })
        .filter_map(|result: Result<(semver::Version, git2::Commit<'_>)>| {
            result.ok()
        });

    let tag_string = tagged_commits
        .map(|commit| {
            format!("{} = {}", commit.0, commit.1.id())
        })
        .reduce(|a, b| {
            format!("{}, {}", a, b)
        })
        .unwrap_or(String::from(""));

    Ok(tag_string)
}

fn get_commit<'a>(repository: &'a git2::Repository, tag_name: &'a str) -> Result<git2::Commit<'a>> {
    let object = repository.revparse_single(&format!("refs/tags/{}", tag_name))?;
    Ok(object.peel_to_commit()?)
}
