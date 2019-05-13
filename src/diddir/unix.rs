use std::io;
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;

static PERMISSIONS_MASK: u32 = 0o777;
static DIR_PERMISSIONS: u32 = 0o700;
static FILE_PERMISSIONS: u32 = 0o600;

pub struct DIDDirSys();

impl DIDDirSys {

    pub fn set_permission(path: &Path) -> io::Result<()> {
        let metadata = fs::metadata(path)?;
        let mut permissions = metadata.permissions();
        let correct_permissions = if path.is_dir() {
            DIR_PERMISSIONS
        } else {
            FILE_PERMISSIONS
        };
        permissions.set_mode(correct_permissions);
        fs::set_permissions(path, permissions)?;
        Ok(())
    }

    pub fn check_permission(path: &Path) -> io::Result<()> {
        let metadata = fs::metadata(path)?;
        let permissions = if path.is_dir() {
            DIR_PERMISSIONS
        } else {
            FILE_PERMISSIONS
        };

        if metadata.mode() & PERMISSIONS_MASK != permissions {
            return Err(io::Error::new(io::ErrorKind::Other,
                format!("Invalid permissions ({}) on: {}",
                metadata.mode(), path.to_str().unwrap())));
        }
        Ok(())
    }

}
