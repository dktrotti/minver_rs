use anyhow::{Result, anyhow};
use std::fmt;
use regex::Regex;

#[derive(Debug)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    prerelease: Option<String>,
    build_metadata: Option<String>
}

impl Version {
    pub fn parse(version: &str) -> Result<Version> {
        // Regex taken from https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
        let pattern = "^(?P<major>0|[1-9]\\d*)\\.(?P<minor>0|[1-9]\\d*)\\.(?P<patch>0|[1-9]\\d*)(?:-(?P<prerelease>(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$";

        let re = Regex::new(pattern).unwrap();
        let captures = re.captures(version).ok_or(anyhow!("Version is not a valid semver 2.0 version: {}", version))?;

        Ok(Version {
            major: captures.name("major").unwrap().as_str().parse().unwrap(),
            minor: captures.name("minor").unwrap().as_str().parse().unwrap(),
            patch: captures.name("patch").unwrap().as_str().parse().unwrap(),
            prerelease: captures.name("prerelease").map(|m| { String::from(m.as_str()) }),
            build_metadata: captures.name("buildmetadata").map(|m| { String::from(m.as_str()) })
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}{}{}",
            self.major,
            self.minor,
            self.patch,
            self.prerelease.as_ref().map(|s| { format!("-{}", s) }).unwrap_or_default(),
            self.build_metadata.as_ref().map(|s| { format!("+{}", s) }).unwrap_or_default(),
        )
    }
}