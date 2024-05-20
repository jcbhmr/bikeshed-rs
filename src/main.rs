use std::error::Error;

fn extract_it() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    const ARCHIVE_BYTES: &[u8] = include_bytes!("../bikeshed-x86_64-pc-windows-msvc.zip");
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    const ARCHIVE_BYTES: &[u8] = include_bytes!("../bikeshed-x86_64-apple-darwin.tar.gz");
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    const ARCHIVE_BYTES: &[u8] = include_bytes!("../bikeshed-arm64-apple-darwin.tar.gz");
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    const ARCHIVE_BYTES: &[u8] = include_bytes!("../bikeshed-x86_64-unknown-linux-gnu.tar.gz");

    let dest = dirs::data_local_dir().ok_or("no data local dir")?.join("bikeshed-rs/".to_owned() + env!("CARGO_PKG_VERSION"));

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

fn main() -> Result<(), Box<dyn Error>> {
    extract_it()?;

    let dest = dirs::data_local_dir().ok_or("no data local dir")?.join("bikeshed-rs/".to_owned() + env!("CARGO_PKG_VERSION"));
    let exe_ext = if cfg!(windows) { ".exe" } else { "" };
    let bikeshed = dest.join("bikeshed".to_owned() + exe_ext);

    let current_exe = std::env::current_exe()?;
    let mut old_exe = current_exe.as_os_str().to_owned();
    old_exe.push(".old");
    old_exe.push(exe_ext);
    #[cfg(unix)]
    {
        std::fs::rename(&current_exe, &old_exe)?;
        std::os::unix::fs::symlink(&bikeshed, &current_exe)?;
        std::fs::remove_file(old_exe)?;
    }
    #[cfg(windows)]
    {
        std::fs::rename(&current_exe, &old_exe)?;
        std::os::windows::fs::symlink_file(&bikeshed, &current_exe)?;
    }

    let mut cmd = std::process::Command::new(bikeshed);
    let args = std::env::args_os().into_iter().collect::<Vec<_>>();
    cmd.args(&args[1..]);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.arg0(&args[0]);
        let error = cmd.exec();
        return Err(Box::new(error));
    }
    #[cfg(not(unix))]
    {
        let status = cmd.status()?;
        std::process::exit(status.code().ok_or("no code")?);
    }
}
