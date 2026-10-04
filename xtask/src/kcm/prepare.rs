use crate::options;
use std::process;

pub fn prepare(kernel: options::Kernel, jobs: Option<u32>) -> anyhow::Result<()> {
    match kernel {
        options::Kernel::Noop => {}
        options::Kernel::FaceId => {
            for bin in ["fetch-models", "migrate"] {
                let mut prepare_command = process::Command::new("cargo");
                prepare_command.args(["run", "--package", "face-id-kernel", "--bin", bin]);
                if let Some(jobs) = jobs {
                    prepare_command.arg(format!("-j{}", jobs));
                }
                crate::sh::run_sh(prepare_command)?;
            }
        }
    }
    Ok(())
}
