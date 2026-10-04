use crate::kcm;
use std::process;

pub fn uninstall() -> anyhow::Result<()> {
    let mut uninstall_command = process::Command::new("sudo");
    let uninstall_so =
        crate::directories::install_directory().join(format!("kcm_{}.so", kcm::consts::LIB_NAME));
    uninstall_command.args(["rm".as_ref(), "-f".as_ref(), uninstall_so.as_os_str()]);
    crate::sh::run_sh(uninstall_command)?;
    Ok(())
}
