fn extract_it() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    const ARCHIVE_BYTES: &[u8] = include_bytes!("bikeshed-x86_64-pc-windows-msvc.zip");
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    const ARCHIVE_BYTES: &[u8] = include_bytes!("bikeshed-x86_64-apple-darwin.tar.gz");
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    const ARCHIVE_BYTES: &[u8] = include_bytes!("bikeshed-arm64-apple-darwin.tar.gz");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    const ARCHIVE_BYTES: &[u8] = include_bytes!("bikeshed-x86_64-unknown-linux-gnu.tar.gz");

    let xdg_dirs = xdg::BaseDirectories::with_prefix("bikeshed-rs/".to_owned() + env!("CARGO_PKG_VERSION"))?;
    let dest = xdg_dirs.get_data_home();

    if !dest.exists() {
        let ext = if cfg!(windows) {
            ".zip"
        } else {
            ".tar.gz"
        };
        let archive = std::io::Cursor::new(ARCHIVE_BYTES);
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    extract_it()
}
