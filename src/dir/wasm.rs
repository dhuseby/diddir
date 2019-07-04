pub struct DIDDirSys();

impl DIDDirSys {

    fn set_permission(path: &Path) -> io::Result<()> {
        // No-op for now
        Ok(())
    }

    fn check_permission(path: &Path) -> io::Result<()> {
        // No-op for now
        Ok(())
    }

}
