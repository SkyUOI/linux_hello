use std::{ops::Not as _, process};

use anyhow::{Context, bail};

pub fn run_sh(mut command: process::Command) -> anyhow::Result<()> {
    let status = command
        .status()
        .with_context(|| format!("cannot run command: {command:?}"))?;
    if status.success().not() {
        bail!("command '{command:?}' failed: {status}");
    }
    Ok(())
}
