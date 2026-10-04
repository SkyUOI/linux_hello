use crate::{kcm, options};
use std::process;

pub fn build(kernel: options::Kernel, release: bool, jobs: Option<u32>) -> anyhow::Result<()> {
    let mut build_command = process::Command::new("cargo");
    build_command.args([
        "build",
        "-p",
        kcm::consts::PKG_NAME,
        "--no-default-features",
        "--features",
        kernel.feature(),
    ]);
    if let Some(jobs) = jobs {
        build_command.arg(format!("-j{}", jobs));
    }
    if release {
        build_command.arg("--release");
    }
    crate::sh::run_sh(build_command)?;
    let original_so = crate::directories::target_directory(release)?
        .join(format!("lib{}.so", kcm::consts::LIB_NAME));
    let target_so = crate::directories::target_directory(release)?
        .join(format!("kcm_{}.so", kcm::consts::LIB_NAME));
    std::fs::copy(original_so, target_so)?;
    Ok(())
}
