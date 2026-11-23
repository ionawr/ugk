use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Deserialize, Serialize, Debug)]
pub struct KeyringCredentials(HashMap<String, String>);

impl KeyringCredentials {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn load(secretfile: impl AsRef<Path>) -> Result<Self> {
        let secretfile = secretfile.as_ref();
        let output = Command::new("gpg")
            .arg("--decrypt")
            .arg(secretfile.as_os_str())
            .output()?;
        let decrypted = String::from_utf8(output.stdout)?;

        Ok(toml::from_str::<KeyringCredentials>(&decrypted)?)
    }

    pub fn save(&self, secretfile: impl AsRef<Path>, recipient: &str) -> Result<()> {
        let toml = toml::to_string(&self)?;
        let secretfile = secretfile.as_ref();
        let mut child = Command::new("gpg")
            .arg("--encrypt")
            .arg("-o")
            .arg(secretfile.as_os_str())
            .arg("-a")
            .arg("-r")
            .arg(recipient)
            .arg("--yes")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let mut stdin = child.stdin.take().expect("stdin is piped");
        std::thread::spawn(move || {
            stdin
                .write_all(toml.as_bytes())
                .expect("writing to child stdin should succeed");
        });

        child.wait_with_output()?;
        Ok(())
    }

    pub fn init(&mut self, secretfile: impl AsRef<Path>) -> Result<File> {
        let secretfile = secretfile.as_ref();
        if let Some(parent) = secretfile.parent() {
            create_dir_all(parent)?;
            File::create_new(secretfile).map_err(anyhow::Error::from)
        } else {
            bail!("given path is incorrect");
        }
    }

    pub fn add(&mut self, keyring: &str, password: &str) -> Result<()> {
        self.0.insert(String::from(keyring), String::from(password));
        Ok(())
    }

    pub fn remove(&mut self, keyring: &str) -> Result<String> {
        self.0
            .remove(keyring)
            .with_context(|| format!("no keyring found with the name {keyring}"))
    }
}
