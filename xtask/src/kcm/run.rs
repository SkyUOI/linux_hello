use crate::{kcm, options};
use std::process;

pub fn run(kernel: options::Kernel, release: bool, jobs: Option<u32>) -> anyhow::Result<()> {
    kcm::build::build(kernel, release, jobs)?;
    let mut run_command = process::Command::new("kcmshell6");
    run_command
        .arg(format!("kcm_{}", kcm::consts::LIB_NAME))
        .envs([
            ("QML_DISABLE_DISK_CACHE", "1".as_ref()),
            (
                "QT_PLUGIN_PATH",
                crate::directories::target_directory(release)?.as_os_str(),
            ),
        ]);
    crate::sh::run_sh(run_command)?;
    Ok(())
}
