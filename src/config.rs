extern crate directories;

use directories::ProjectDirs;
use std::default::Default;
use std::path::{Path, PathBuf};

static QUALIFIER: &'static str = "org";
static ORGANIZATION: &'static str = "linuxfoundation";
static APPLICATION: &'static str = "diddir";
static ALIASES: &'static str = "aliases";
static TMP: &'static str = "tmp";

#[derive(Debug, PartialEq)]
pub struct Config {
    root: PathBuf,
    aliases: PathBuf,
    tmp: PathBuf
}

impl Default for Config {
    fn default() -> Self {
        let dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).unwrap();
        let root = dirs.data_local_dir().to_path_buf();
        let mut aliases = root.clone();
        aliases.push(ALIASES);
        let mut tmp = root.clone();
        tmp.push(TMP);

        Config { 
            root: root,
            aliases: aliases,
            tmp: tmp
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn with_path(path: &Path) -> Self {
        let root = PathBuf::from(path);
        let mut aliases = root.clone();
        aliases.push(ALIASES);
        let mut tmp = root.clone();
        tmp.push(TMP);

        Config { 
            root: root,
            aliases: aliases,
            tmp: tmp
        }
    }

    pub fn root_dir(&self) -> &Path {
        self.root.as_path()
    }

    pub fn aliases_dir(&self) -> &Path {
        self.aliases.as_path()
    }

    pub fn tmp_dir(&self) -> &Path {
        self.tmp.as_path()
    }
}
