mod matching;
mod mime;
mod runtime_assets;

pub mod contents {
    pub mod json;
}

use rust_embed::Embed;

pub use {matching::get, runtime_assets::RuntimeAssets};

#[derive(Embed)]
#[folder = "../public"]
struct Public;
