use std::env;

use anyhow::Result;
use git2::Repository;

use minver_rs::Version;

fn main() {
    match get_version() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Error: {}", e),
    }
}

fn get_version() -> Result<Version> {
    let dir = env::current_dir()?;
    let repo = Repository::open(dir.as_path())?;
    minver_rs::get_version(&repo)
}
