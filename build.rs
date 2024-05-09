use std::error::Error;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("bikeshed-rs/".to_owned() + VERSION)?;
    let dest = xdg_dirs.get_data_home();

    if !dest.exists() {
        let target = build_target::target_triple()?;
        let ext = if target.contains("windows") {
            ".zip"
        } else {
            ".tar.gz"
        };
        let archive = std::fs::File::open("bikeshed-".to_owned() + &target + ext)?;
        if ext == ".zip" {
            let mut a = zip::ZipArchive::new(archive)?;
            a.extract(&dest)?;
        } else {
            let gz = flate2::read::GzDecoder::new(archive);
            let mut a = tar::Archive::new(gz);
            a.unpack(&dest)?;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(
                dest.join("bikeshed"),
                std::fs::Permissions::from_mode(0o755),
            )?;
        }
    }
    
    Ok(())
}
