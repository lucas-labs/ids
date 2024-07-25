pub mod actions {
    mod action;
    pub use action::*;

    pub mod help;
    pub mod version;
}

mod shared {
    pub mod config;
    pub mod print;
    pub mod versioning_data;
}

pub mod git;

pub use shared::{config::*, print, versioning_data::*};
