pub struct DIDDirSys();

impl DIDDirSys {

    fn set_permission(path: &Path) -> io::Result<()> {
        // No-op until Rust stdlib supports Windows permission constants
        Ok(())
    }

    fn check_permission(path: &Path) -> io::Result<()> {
        // No-op until Rust stdlib supports Windows permission constants
        Ok(())
    }

}
