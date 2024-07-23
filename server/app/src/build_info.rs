//! ## Build Info
//!
//! This module provides functionality to access and display build-related information for the
//! project (name, version, dependencies, git metadata, etc).
//!
//! The information is generated at build time.

use cli::VersioningData;

include!(concat!(env!("OUT_DIR"), "/built.rs"));

pub fn get() -> VersioningData {
    VersioningData {
        name: self::PKG_NAME,
        version: self::PKG_VERSION,
        git_hash: self::GIT_COMMIT_HASH.unwrap_or(""),
        git_shash: self::GIT_COMMIT_HASH_SHORT.unwrap_or(""),
    }
}
