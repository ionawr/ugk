use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Deserialize, Serialize)]
pub struct KeyringCredentials(HashMap<String, String>);

impl KeyringCredentials {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn load(secretfile: &Path) -> Result<Self> {
        let output = Command::new("gpg")
            .arg("--decrypt")
            .arg(secretfile.as_os_str())
            .output()?;
        let decrypted = String::from_utf8(output.stdout)?;

        Ok(toml::from_str::<KeyringCredentials>(&decrypted)?)
    }

    pub fn save(&self, secretfile: &Path, recipient: &str) -> Result<()> {
        let toml = toml::to_string(&self)?;
        let mut child = Command::new("gpg")
            .arg("--encrypt")
            .arg("-o")
            .arg(secretfile.as_os_str())
            .arg("-a")
            .arg("-r")
            .arg(recipient)
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

    pub fn add(&mut self, keyring: String, password: String) -> Result<()> {
        self.0.insert(keyring, password);
        Ok(())
    }

    pub fn init(&mut self, secretfile: &Path) -> Result<File> {
        File::create_new(secretfile).map_err(anyhow::Error::from)
    }
}
