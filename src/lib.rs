mod build_utils;
mod minver_core;
mod semver;

pub use minver_core::get_version;
pub use minver_core::Version;

pub use build_utils::default_build_action;
pub use build_utils::update_package_version;
pub use build_utils::UPDATE_VERSION_VAR;
