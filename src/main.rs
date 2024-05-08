use std::error::Error;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("bikeshed-rs/".to_owned() + VERSION)?;
    let dest = xdg_dirs.get_data_home();

    let exe_ext = if cfg!(windows) { ".exe" } else { "" };
    let status = std::process::Command::new(dest.join("bikeshed".to_owned() + exe_ext)).status()?;
    if status.success() {
        Ok(())
    } else {
        Err("not success".into())
    }
}
