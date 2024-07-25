use std::path::PathBuf;

pub struct CliConfig {
    pub path: PathBuf,
    pub port: u32,
    pub host: String,
    pub spa: bool,
    pub serve_ui: bool,
}
