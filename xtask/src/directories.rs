use anyhow::Context as _;
use std::path;

pub fn workspace_directory() -> anyhow::Result<path::PathBuf> {
    let xtask_directory = path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    xtask_directory
        .parent()
        .map(ToOwned::to_owned)
        .context("xtask has no parent directory")
}

pub fn target_directory(release: bool) -> anyhow::Result<path::PathBuf> {
    let workspace_directory = workspace_directory()?;
    Ok(workspace_directory
        .join("target")
        .join(if release { "release" } else { "debug" }))
}

pub fn install_directory() -> path::PathBuf {
    path::Path::new("/")
        .join("usr")
        .join("lib")
        .join("qt6")
        .join("plugins")
        .join("plasma")
        .join("kcms")
        .join("systemsettings")
}
