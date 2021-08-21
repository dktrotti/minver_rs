// use git2;

// pub fn get_tag(path: &std::path::Path, pattern: &str) -> Result<String, git2::Error> {
//     let repo = git2::Repository::open_ext(path, git2::RepositoryOpenFlags::NO_SEARCH, std::iter::empty::<String>())?;
//     let tags = repo.tag_names(Some(pattern))?;

//     return String::from("1.0.0")
// }