#[macro_use]
extern crate cfg_if;

pub use self::config::Config;
pub mod config;

pub use self::diddir::DIDDir;
pub mod diddir;

pub use self::did::*;
pub mod did;
