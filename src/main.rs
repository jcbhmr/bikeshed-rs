use std::error::Error;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn Error>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("bikeshed-rs/".to_owned() + VERSION)?;
    let dest = xdg_dirs.get_data_home();

    let exe_ext = if cfg!(windows) { ".exe" } else { "" };
    let mut cmd = std::process::Command::new(dest.join("bikeshed".to_owned() + exe_ext));
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
    cmd;
    return Ok(());
}
