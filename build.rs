use std::error::Error;

#[cfg(target_os = "windows")]
fn f() {}

#[cfg(target_os = "linux")]
fn f() {}

#[cfg(target_os = "macos")]
fn f() {}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("bikeshed-rs/".to_owned() + VERSION)?;
    let dest = xdg_dirs.get_data_home();

    let target = "x86_64-unknown-linux-gnu";
    let ext = ".tar.gz";
    let archive = std::fs::File::open("bikeshed-".to_owned() + target + ext)?;
    if ext == ".zip" {
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
    Ok(())
}
