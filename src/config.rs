use anyhow::Result;
use std::env::var_os;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub fn default_secretfile_path() -> Result<PathBuf> {
    let secretfile_path = var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .ok_or_else(|| {
            Error::new(
                ErrorKind::NotFound,
                "neither XDG_CONFIG_HOME nor HOME is set",
            )
        })?;

    Ok(secretfile_path.join("ugk").join("secret.toml"))
}
