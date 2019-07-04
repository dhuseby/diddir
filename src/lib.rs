#[macro_use]
extern crate cfg_if;

pub use self::config::Config;
pub mod config;

pub use self::dir::DIDDir;
pub mod dir;

pub use self::doc::*;
pub mod doc;
