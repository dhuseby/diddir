#[macro_use]
extern crate cfg_if;

pub mod config;
pub use self::config::Config;

pub mod diddir;
pub use self::diddir::DIDDir;


