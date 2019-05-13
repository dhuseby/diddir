use crate::Config;
use rand;
use rand::distributions::{Alphanumeric, Distribution};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

cfg_if! {
    if #[cfg(unix)] {
        pub mod unix;
        use self::unix::DIDDirSys;
    } else if #[cfg(target_os = "windows")] {
        pub mod windows;
        use self::windows::DIDDirSys;
    } else if #[cfg(wasm)] {
        pub mod wasm;
        use self::wasm::DIDDirSys;
    }
}

#[derive(Debug, PartialEq)]
pub struct DIDDir<'a> {
    config: &'a Config,
    ids: HashMap<String, PathBuf>,
    aliases: HashMap<String, String>
}

impl<'a> DIDDir<'a> {

    pub fn open(config: &'a Config) -> io::Result<Self>  {

        Self::check_dirs_exist(config)?;
        Self::check_permissions(config.root_dir())?;

        Ok(DIDDir { 
            config: config,
            ids: Self::read_ids(config.root_dir())?,
            aliases: Self::read_aliases(config.aliases_dir())?
        })
    }

    pub fn init(config: &'a Config) -> io::Result<Self> {
        let dirs = vec![config.root_dir(), config.aliases_dir(), config.tmp_dir()];

        for d in dirs {
            if d.is_dir() {
                if fs::read_dir(d)?.count() > 0 {
                    return Err(io::Error::new(io::ErrorKind::AlreadyExists,
                        format!("Error creating (already exists): {}",
                        d.to_str().unwrap())));
                }
            } else {
                // create the dirs
                fs::create_dir_all(d)?;
            }
        }

        // set the permissions correctly
        Self::set_permissions(config.root_dir())?;

        // open the diddir
        Self::open(config)
    }

    pub fn open_or_init(config: &'a Config) -> io::Result<Self> {
        match Self::open(config) {
            Ok(diddir) => Ok(diddir),
            _ => Self::init(config)
        }
    }

    pub fn save_identity(&mut self, pkid: &String, data: &String) -> io::Result<()> {
        // get the path to a tmp file
        let path = self.get_tmp_file_path(pkid)?;

        // create the file, store the data
        {
            let mut file = fs::File::create(path.as_path())?;
            file.write_all(data.as_bytes())?;
        }

        // set the permissions
        DIDDirSys::set_permission(&path)?;

        // atomically move the file from the tmp dir to the root dir
        let mut root_path = PathBuf::new();
        root_path.push(self.config.root_dir());
        root_path.push(pkid);
        fs::rename(path, root_path)?;

        // reload our ids and aliases state
        self.reload()?;

        Ok(())
    }

    pub fn get_identity(&self, pkid: &String) -> io::Result<String> {
        if let Some(path) = self.ids.get(pkid) {
            let mut file = fs::File::open(path.as_path())?;
            let mut data = String::new();
            file.read_to_string(&mut data)?;
            Ok(data)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound,
                format!("No identity file found for: {}", pkid)))
        }
    }

    pub fn get_identities(&self) -> Option<Vec<String>> {
        let mut ids = Vec::new();
        for k in self.ids.keys() {
            ids.push(k.to_owned());
        }
        if ids.len() == 0 {
            return None;
        }
        Some(ids)
    }

    pub fn remove_identity(&mut self, pkid: &String) -> io::Result<()> {
        // generate a path to a .deleted-XXXXX file in tmp dir
        let del_path = self.get_tmp_file_path(&".deleted".to_string())?;

        // calculate the path to the DID doc
        let mut root_path = PathBuf::new();
        root_path.push(self.config.root_dir());
        root_path.push(pkid);

        // if the DID doc doesn't exist, then throw an error
        if !root_path.exists() {
            return Err(io::Error::new(io::ErrorKind::Other, 
                       "Identity file does not exist"));
        }

        // remove all aliases
        if let Some(aliases) = self.get_aliases(pkid) {
            for alias in aliases {
                self.remove_alias(&alias)?;
            }
        } else {}

        // atomically move the alias file to tmp dir and delete it
        fs::rename(root_path, &del_path)?;
        fs::remove_file(del_path)?;

        // reload out ids and aliases state
        self.reload()?;

        Ok(())
    }

    pub fn get_pkid_from_alias(&self, alias: &String) -> io::Result<String> {
        if let Some(pkid) = self.aliases.get(alias) {
            Ok(pkid.to_owned())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound,
                format!("No identity found for: {}", alias)))
        }
    }

    pub fn save_alias(&mut self, alias: &String, pkid: &String) -> io::Result<()> {
        // get the path to a tmp file
        let path = self.get_tmp_file_path(alias)?;

        // create the file, store the data
        {
            let mut file = fs::File::create(path.as_path())?;
            file.write_all(pkid.as_bytes())?;
        }

        // set the permissions
        DIDDirSys::set_permission(&path)?;

        // atomically move the file from the tmp dir to the ids dir
        let mut alias_path = PathBuf::new();
        alias_path.push(self.config.aliases_dir());
        alias_path.push(alias);
        fs::rename(path, alias_path)?;

        // reload our ids and aliases state
        self.reload()?;

        Ok(())
    }

    pub fn remove_alias(&mut self, alias: &String) -> io::Result<()> {
        // generate a path to a .deleted-XXXXX file in tmp dir
        let del_path = self.get_tmp_file_path(&".deleted".to_string())?;

        // calculate the path to the alias file
        let mut alias_path = PathBuf::new();
        alias_path.push(self.config.aliases_dir());
        alias_path.push(alias);

        // if the alias file doesn't exist, then throw an error
        if !alias_path.exists() {
            return Err(io::Error::new(io::ErrorKind::Other, 
                       "Alias file does not exist"));
        }

        // atomically move the alias file to tmp dir and delete it
        fs::rename(alias_path, &del_path)?;
        fs::remove_file(del_path)?;

        // reload out ids and aliases state
        self.reload()?;

        Ok(())
    }

    pub fn get_aliases(&self, pkid: &String) -> Option<Vec<String>> {
        let mut aliases: Vec<String> = Vec::new();
        
        for (alias, id) in self.aliases.iter() {
            if id == pkid {
                aliases.push(alias.to_owned());
            }
        }
        if aliases.len() == 0 {
            return None;
        }

        Some(aliases)
    }

    fn get_tmp_file_path(&self, name: &String) -> io::Result<PathBuf> {
        for _ in 1..100 {
            // generate a random string to append to the name
            let mut rng = rand::thread_rng();
            let rnd_ext: String = Alphanumeric.sample_iter(&mut rng).take(6).collect();

            // calculate the full path
            let mut tmp_path = PathBuf::new();
            tmp_path.push(self.config.tmp_dir());
            tmp_path.push(format!("{}-{}", name, rnd_ext));

            if !tmp_path.exists() {
                return Ok(tmp_path);
            }
        }
        Err(io::Error::new(io::ErrorKind::Other, 
            "Could not calculate unique filename for tmp file."))
    }

    fn check_dirs_exist(config: &'a Config) -> io::Result<()> {
        let dirs = vec![config.root_dir(), config.aliases_dir(), config.tmp_dir()];

        for d in dirs {
            if !d.is_dir() {
                return Err(io::Error::new(io::ErrorKind::NotFound,
                    format!("No DIDDir directory at: {}",
                    d.to_str().unwrap())));
            }
        }

        Ok(())
    }

    fn set_permissions(path: &Path) -> io::Result<()> {
        DIDDirSys::set_permission(path)?;
        
        if path.is_dir() {
            // check all contents of directory
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                Self::set_permissions(&entry.path())?;
            }
        }
        
        Ok(())
    }

    fn check_permissions(path: &Path) -> io::Result<()> {
        DIDDirSys::check_permission(path)?;
        
        if path.is_dir() {
            // check all contents of directory
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                Self::check_permissions(&entry.path())?;
            }
        }
        
        Ok(())
    }

    fn reload(&mut self) -> io::Result<()> {
        self.ids = Self::read_ids(self.config.root_dir())?;
        self.aliases = Self::read_aliases(self.config.aliases_dir())?;
        Ok(())
    }

    fn read_ids(path: &Path) -> io::Result<HashMap<String, PathBuf>> {
        if !path.is_dir() {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                format!("No DIDDir at: {}",
                path.to_str().unwrap())));
        }

        let mut ids = HashMap::new();

        println!("-----");
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if !entry.metadata()?.is_dir() {
                let p = entry.path();
                println!("id: {}", p.to_str().unwrap());
                let pkid = p.file_name().unwrap().to_os_string().into_string().unwrap();
                ids.insert(pkid, PathBuf::from(entry.path()));
            }
        }
        println!("=====");

        Ok(ids)
    }

    fn read_aliases(path: &Path) -> io::Result<HashMap<String, String>> {
        if !path.is_dir() {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                format!("No DIDDir at: {}",
                path.to_str().unwrap())));
        }

        let mut aliases = HashMap::new();

        println!("-----");
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if !entry.metadata()?.is_dir() {
                let p = entry.path();
                println!("alias: {}", p.to_str().unwrap());
                let alias = p.file_name().unwrap().to_os_string().into_string().unwrap();
                let pkid = fs::read_to_string(entry.path())?.trim().to_owned();
                aliases.insert(alias, pkid);
            }
        }
        println!("=====");

        Ok(aliases)
    }
}
