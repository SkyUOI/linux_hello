use crate::{kcm, options};
use std::process;

pub fn install(kernel: options::Kernel, jobs: Option<u32>) -> anyhow::Result<()> {
    kcm::build::build(kernel, true, jobs)?;
    let mut install_command = process::Command::new("sudo");
    let original_so = crate::directories::target_directory(true)?
        .join(format!("kcm_{}.so", kcm::consts::LIB_NAME));
    let target_so =
        crate::directories::install_directory().join(format!("kcm_{}.so", kcm::consts::LIB_NAME));
    install_command.args([
        "install".as_ref(),
        original_so.as_os_str(),
        target_so.as_os_str(),
    ]);
    crate::sh::run_sh(install_command)?;
    Ok(())
}
