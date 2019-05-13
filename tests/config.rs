extern crate diddir;

mod common {
    use diddir::Config;
    use std::path::PathBuf;

    static ALIASES: &'static str = "aliases";
    static TMP: &'static str = "tmp";

    pub fn config_default(root: &PathBuf) {
	let mut aliases = root.clone();
	aliases.push(ALIASES);
        let mut tmp = root.clone();
        tmp.push(TMP);
        let config = Config::default();
        assert_eq!(root.as_path(), config.root_dir());
        assert_eq!(aliases.as_path(), config.aliases_dir());
        assert_eq!(tmp.as_path(), config.tmp_dir());
    }

    pub fn config_with_path(root: &PathBuf) {
	let mut aliases = root.clone();
	aliases.push(ALIASES);
	let mut tmp = root.clone();
        tmp.push(TMP);
        let config = Config::with_path(root.as_path());
        assert_eq!(root.as_path(), config.root_dir());
        assert_eq!(aliases.as_path(), config.aliases_dir());
        assert_eq!(tmp.as_path(), config.tmp_dir());
    }
}


#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "dragonfly",
    target_os = "bitrig"
))]
mod unix_test {
    use crate::common;
    use std::path::PathBuf;

    #[test]
    fn config_default() {
        let root: PathBuf =
        [ 
            env!("HOME"),
            ".local",
            "share",
            "diddir"
        ].iter().collect();

        common::config_default(&root);
    }

    #[test]
    fn config_with_path() {
        let root: PathBuf =
        [ 
            env!("HOME"),
            ".diddir"
        ].iter().collect();

        common::config_with_path(&root);
    }
}


#[cfg(target_os = "macos")]
mod mac_test {
    use crate::common;
    use std::path::PathBuf;

    #[test]
    fn config_default() {
        let root: PathBuf =
        [ 
            env!("HOME"),
            "Library",
            "Application",
            "Support",
            "org.linuxfoundation.diddir" 
        ].iter().collect();

        common::config_default(&root);
    }

    #[test]
    fn config_with_path() {
        let root: PathBuf =
        [ 
            env!("HOME"),
            ".diddir"
        ].iter().collect();

        common::config_with_path(&root);
    }
}

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "windows")]
mod windows_test {
    use diddir::Config;
    use std::path::PathBuf;
    use winapi::shared::winerror;
    use winapi::um::knownfolders;
    use winapi::um::combaseapi;
    use winapi::um::shlobj;
    use winapi::um::shtypes;
    use winapi::um::winbase;
    use winapi::um::winnt;

	fn known_folder(folder_id: shtypes::REFKNOWNFOLDERID) -> Option<PathBuf> {
		unsafe {
			let mut path_ptr: winnt::PWSTR = std::ptr::null_mut();
			let result = shlobj::SHGetKnownFolderPath(folder_id, 0, std::ptr::null_mut(), &mut path_ptr);
			if result == winerror::S_OK {
				let len = winbase::lstrlenW(path_ptr) as usize;
				let path = std::slice::from_raw_parts(path_ptr, len);
				let ostr: std::ffi::OsString = std::os::windows::ffi::OsStringExt::from_wide(path);
				combaseapi::CoTaskMemFree(path_ptr as *mut winapi::ctypes::c_void);
				Some(PathBuf::from(ostr))
			} else {
				None
			}
		}
	}

    #[test]
    fn config_default() {
        let root: PathBuf =
        [ 
            known_folder(&knownfolders::FOLDERID_LocalAppData).unwrap().as_str(),
            "linuxfoundation", 
            "diddir",
            "data"
        ].iter().collect();

        common::config_default(&root);
    }

    #[test]
    fn config_with_path() {
        let root: PathBuf =
        [ 
            known_folder(&knownfolders::FOLDERID_LocalAppData).unwrap().as_str(),
            "linuxfoundation", 
            "diddir",
            "data"
        ].iter().collect();
        let path2 = path.clone();

        common::config_with_path(&root);
    }
}
