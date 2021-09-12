use std::env;

use anyhow::Result;
use git2::Repository;
use log::Level;

use minver_rs::Version;

fn main() {
    if let Err(e) = simple_logger::init_with_level(Level::Warn) {
        println!("Failed to initialize log: {}", e);
    }

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
