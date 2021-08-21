use anyhow::{Result, anyhow};
use regex::Regex;

use std::fmt;
use std::cmp::Ordering;

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn cmp_precedence(&self, other: &Self) -> Ordering {
        let partial_version = (self.major, self.minor, self.patch);
        let other_partial_version = (other.major, other.minor, other.patch);

        match partial_version.cmp(&other_partial_version) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                match (self.prerelease.as_ref(), other.prerelease.as_ref()) {
                    (None, None) => Ordering::Equal,
                    (Some(_), None) => Ordering::Less,
                    (None, Some(_)) => Ordering::Greater,
                    // TODO: This prerelease comparison doesn't quite follow semver 2.0
                    (Some(s), Some(o)) => s.cmp(&o)
                }
            }
        }
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

#[cfg(test)]
mod test {
    use super::*;

    fn create_version(
            major: u32,
            minor: u32,
            patch: u32,
            prerelease: Option<String>,
            build_metadata: Option<String>) -> Version {
        Version {
            major,
            minor,
            patch,
            prerelease,
            build_metadata
        }
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            create_version(1, 2, 3, None, None),
            Version::parse("1.2.3").unwrap());
        assert_eq!(
            create_version(1, 2, 3, Some(String::from("alpha.1.1")), None),
            Version::parse("1.2.3-alpha.1.1").unwrap());
        assert_eq!(
            create_version(1, 2, 3, None, Some(String::from("a1b2c3"))),
            Version::parse("1.2.3+a1b2c3").unwrap());
        assert_eq!(
            create_version(1, 2, 3, Some(String::from("alpha.1.1")), Some(String::from("a1b2c3"))),
            Version::parse("1.2.3-alpha.1.1+a1b2c3").unwrap());
        
        assert!(Version::parse("v1.2.3").is_err())
    }

    #[test]
    fn test_precedence_comparison() {
        let mut versions = vec!(
            create_version(1, 4, 4, None, None),
            create_version(2, 2, 3, Some(String::from("beta")), None),
            create_version(1, 3, 5, None, None),
            create_version(2, 2, 3, None, None),
            create_version(1, 3, 6, None, None),
            create_version(2, 2, 3, Some(String::from("alpha")), None),
        );
        let expected_versions = vec!(
            create_version(1, 3, 5, None, None),
            create_version(1, 3, 6, None, None),
            create_version(1, 4, 4, None, None),
            create_version(2, 2, 3, Some(String::from("alpha")), None),
            create_version(2, 2, 3, Some(String::from("beta")), None),
            create_version(2, 2, 3, None, None),
        );

        versions.sort_by(|v1, v2| { v1.cmp_precedence(&v2) });

        assert_eq!(versions, expected_versions);
    }

    #[test]
    fn test_build_metadata_ignored_in_precedence_comparison() {
        let metadata1 = create_version(1, 2, 3, None, Some(String::from("a1b2c3")));
        let metadata2 = create_version(1, 2, 3, None, Some(String::from("d4e5f6")));
        let no_metadata = create_version(1, 2, 3, None, None);

        assert_eq!(Ordering::Equal, metadata1.cmp_precedence(&metadata2));
        assert_eq!(Ordering::Equal, metadata2.cmp_precedence(&no_metadata));
        assert_eq!(Ordering::Equal, no_metadata.cmp_precedence(&metadata1));
    }
}