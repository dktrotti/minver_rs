use anyhow::{anyhow, Result};
use regex::Regex;
use strum_macros::{Display, EnumString};

use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub prerelease: Option<String>,
    pub build_metadata: Option<String>,
}

#[derive(EnumString, Display, Debug)]
pub enum Level {
    Major,
    Minor,
    Patch,
}

impl Version {
    pub fn parse(version: &str) -> Result<Version> {
        log::trace!("Parsing version: {}", version);
        // Regex taken from https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
        let pattern = "^(?P<major>0|[1-9]\\d*)\\.(?P<minor>0|[1-9]\\d*)\\.(?P<patch>0|[1-9]\\d*)(?:-(?P<prerelease>(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$";

        let re = Regex::new(pattern).unwrap();
        let captures = re.captures(version).ok_or(anyhow!(
            "Version is not a valid semver 2.0 version: {}",
            version
        ))?;

        Ok(Version {
            major: captures.name("major").unwrap().as_str().parse().unwrap(),
            minor: captures.name("minor").unwrap().as_str().parse().unwrap(),
            patch: captures.name("patch").unwrap().as_str().parse().unwrap(),
            prerelease: captures
                .name("prerelease")
                .map(|m| String::from(m.as_str())),
            build_metadata: captures
                .name("buildmetadata")
                .map(|m| String::from(m.as_str())),
        })
    }

    pub fn cmp_precedence(&self, other: &Self) -> Ordering {
        log::trace!("Comparing {} and {}", &self, &other);
        let partial_version = (self.major, self.minor, self.patch);
        let other_partial_version = (other.major, other.minor, other.patch);

        match partial_version.cmp(&other_partial_version) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match (self.prerelease.as_ref(), other.prerelease.as_ref()) {
                (None, None) => Ordering::Equal,
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (Some(s), Some(o)) => Version::cmp_prerelease(s, o),
            },
        }
    }

    fn cmp_prerelease(self_prerelease: &str, other_prerelease: &str) -> Ordering {
        log::trace!(
            "Comparing prereleases {} and {}",
            self_prerelease,
            other_prerelease
        );
        let self_parts = self_prerelease.split(".");
        let other_parts = other_prerelease.split(".");

        for (self_part, other_part) in self_parts.zip(other_parts) {
            // Numeric identifiers have lower precedence than alphanumeric identifiers
            let ordering = match (self_part.parse::<u32>(), other_part.parse::<u32>()) {
                (Ok(s), Ok(o)) => s.cmp(&o),
                (Ok(_), Err(_)) => Ordering::Less,
                (Err(_), Ok(_)) => Ordering::Greater,
                (Err(_), Err(_)) => self_part.cmp(other_part),
            };

            // If one of the parts is greater than the other, return that ordering
            if ordering == Ordering::Equal {
                continue;
            } else {
                log::trace!(
                    "Found difference between {} and {}: {:?}",
                    self_part,
                    other_part,
                    ordering
                );
                return ordering;
            }
        }

        // If all of the paired parts are equal, the identifier with more parts takes precedence
        let self_count = self_prerelease.split(".").count();
        let other_count = other_prerelease.split(".").count();
        log::trace!(
            "All prerelease parts are equal, comparing lengths: {} and {}",
            self_count,
            other_count
        );
        self_count.cmp(&other_count)
    }

    pub fn default(default_prerelease: &str) -> Version {
        Version {
            major: 0,
            minor: 0,
            patch: 0,
            prerelease: Some(format!("{}.0", default_prerelease)),
            build_metadata: None,
        }
    }

    pub fn with_height(self, height: u32, default_prerelease: &str) -> Version {
        Version {
            prerelease: Some(format!(
                "{}.{}",
                self.prerelease.unwrap_or(String::from(default_prerelease)),
                height
            )),
            ..self
        }
    }

    pub fn without_metadata(self) -> Version {
        Version {
            build_metadata: None,
            ..self
        }
    }

    pub fn with_appended_metadata(self, metadata: &String) -> Version {
        let new_metadata: String = self
            .build_metadata
            .map(|m| format!("{}.{}", m, metadata))
            .unwrap_or(metadata.clone());
        Version {
            build_metadata: Some(new_metadata),
            ..self
        }
    }

    pub fn with_incremented_level(self, level: &Level) -> Version {
        match level {
            Level::Major => Version {
                major: self.major + 1,
                minor: 0,
                patch: 0,
                ..self
            },
            Level::Minor => Version {
                minor: self.minor + 1,
                patch: 0,
                ..self
            },
            Level::Patch => Version {
                patch: self.patch + 1,
                ..self
            },
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}{}{}",
            self.major,
            self.minor,
            self.patch,
            self.prerelease
                .as_ref()
                .map(|s| { format!("-{}", s) })
                .unwrap_or_default(),
            self.build_metadata
                .as_ref()
                .map(|s| { format!("+{}", s) })
                .unwrap_or_default(),
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
        build_metadata: Option<String>,
    ) -> Version {
        Version {
            major,
            minor,
            patch,
            prerelease,
            build_metadata,
        }
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            create_version(1, 2, 3, None, None),
            Version::parse("1.2.3").unwrap()
        );
        assert_eq!(
            create_version(1, 2, 3, Some(String::from("alpha.1.1")), None),
            Version::parse("1.2.3-alpha.1.1").unwrap()
        );
        assert_eq!(
            create_version(1, 2, 3, None, Some(String::from("a1b2c3"))),
            Version::parse("1.2.3+a1b2c3").unwrap()
        );
        assert_eq!(
            create_version(
                1,
                2,
                3,
                Some(String::from("alpha.1.1")),
                Some(String::from("a1b2c3"))
            ),
            Version::parse("1.2.3-alpha.1.1+a1b2c3").unwrap()
        );

        assert!(Version::parse("v1.2.3").is_err())
    }

    #[test]
    fn test_precedence_comparison() {
        let mut versions = vec![
            create_version(1, 4, 4, None, None),
            create_version(2, 2, 3, Some(String::from("beta")), None),
            create_version(1, 3, 5, None, None),
            create_version(2, 2, 3, None, None),
            create_version(1, 3, 6, None, None),
            create_version(2, 2, 3, Some(String::from("alpha")), None),
        ];
        let expected_versions = vec![
            create_version(1, 3, 5, None, None),
            create_version(1, 3, 6, None, None),
            create_version(1, 4, 4, None, None),
            create_version(2, 2, 3, Some(String::from("alpha")), None),
            create_version(2, 2, 3, Some(String::from("beta")), None),
            create_version(2, 2, 3, None, None),
        ];

        versions.sort_by(|v1, v2| v1.cmp_precedence(&v2));

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

    #[test]
    fn test_prerelease_precedence_comparison() {
        // 1.0.0-alpha < 1.0.0-alpha.1 < 1.0.0-alpha.beta < 1.0.0-beta < 1.0.0-beta.2 < 1.0.0-beta.11 < 1.0.0-rc.1 < 1.0.0.
        let mut versions = vec![
            create_version(1, 0, 0, Some(String::from("beta.11")), None),
            create_version(1, 0, 0, Some(String::from("rc.1")), None),
            create_version(1, 0, 0, Some(String::from("alpha.1")), None),
            create_version(1, 0, 0, None, None),
            create_version(1, 0, 0, Some(String::from("beta.2")), None),
            create_version(1, 0, 0, Some(String::from("alpha.beta")), None),
            create_version(1, 0, 0, Some(String::from("alpha")), None),
            create_version(1, 0, 0, Some(String::from("beta")), None),
        ];
        let expected_versions = vec![
            create_version(1, 0, 0, Some(String::from("alpha")), None),
            create_version(1, 0, 0, Some(String::from("alpha.1")), None),
            create_version(1, 0, 0, Some(String::from("alpha.beta")), None),
            create_version(1, 0, 0, Some(String::from("beta")), None),
            create_version(1, 0, 0, Some(String::from("beta.2")), None),
            create_version(1, 0, 0, Some(String::from("beta.11")), None),
            create_version(1, 0, 0, Some(String::from("rc.1")), None),
            create_version(1, 0, 0, None, None),
        ];

        versions.sort_by(|v1, v2| v1.cmp_precedence(&v2));

        assert_eq!(versions, expected_versions);
    }
}
