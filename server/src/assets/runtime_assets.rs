//! Runtime Assets
//!
//! This module contains the `RuntimeAssets` struct which is used to store assets that are created
//! at runtime. Specifically, the `RuntimeAssets` struct will be used to store the javascript file
//! that contains the current changes in the repository. But it was made to be generic so that it
//! can be used to store any asset that is created at runtime in the future.

use {super::mime, std::collections::HashMap};

#[derive(Clone)]
pub struct Asset {
    /// The content of the asset as a byte vector
    pub content: Vec<u8>,
    /// The mime type of the asset
    pub mime: String,
    /// The last modified date of the asset
    pub date_modified: i64,
}

/// Holds assets that are created at runtime and served by the http asset server under a predefined
/// prefix
pub struct RuntimeAssets {
    /// The prefix under which the assets are served
    prefix: String,

    /// Map containing the files as a collection `path -> Asset`
    files: HashMap<String, Asset>,
}

impl Default for RuntimeAssets {
    fn default() -> Self {
        Self::create("/_ids_runtime")
    }
}

impl RuntimeAssets {
    /// Create a new instance of the RuntimeAssets
    pub fn create(prefix: &str) -> Self {
        Self {
            files: HashMap::new(),
            prefix: prefix.to_string(),
        }
    }

    /// Insert/overwrite a file into the map
    ///
    /// If the path doesn't start with the prefix, the prefix is added to the path automatically
    pub fn insert(&mut self, path: &str, content: Vec<u8>) {
        let path = add_prefix(&self.prefix, path);
        let extension = get_extension(&path.to_string());

        self.files.insert(
            path.clone(),
            Asset {
                content,
                mime: mime::extension_to_mime(extension.as_deref()).to_string(),
                date_modified: time::OffsetDateTime::now_utc().unix_timestamp(),
            },
        );
    }

    /// Get an asset by its path
    ///
    /// If the path doesn't start with the prefix, the prefix is added to the path automatically
    pub fn get(&self, path: String) -> Option<Asset> {
        let path = add_prefix(&self.prefix, &path);
        self.files.get(&path).cloned()
    }

    /// Remove assets prefixed with the given path
    pub fn remove_path(&mut self, path: &str) {
        let path = add_prefix(&self.prefix, path);
        self.files.retain(|key, _| !key.starts_with(&path));
    }

    /// returns the prefix of the assets
    pub fn prefix(&self) -> String {
        self.prefix.clone()
    }
}

/// Get the extension of a file path
fn get_extension(path: &str) -> Option<String> {
    let parts: Vec<&str> = path.split('.').collect();
    if parts.len() > 1 {
        Some(parts.last().unwrap().to_string())
    } else {
        None
    }
}

/// Add a prefix to a path if it doesn't already have it
fn add_prefix(prefix: &str, path: &str) -> String {
    if path.starts_with(prefix) {
        path.to_string()
    } else {
        format!("{}/{}", prefix, path.trim_start_matches('/'))
    }
}
