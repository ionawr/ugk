use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Deserialize, Serialize)]
struct KeyringCredentials(HashMap<String, String>);

impl KeyringCredentials {
    fn load(secretfile: &Path) -> Result<Self> {
        let output = Command::new("gpg")
            .arg("--decrypt")
            .arg(secretfile.as_os_str())
            .output()?;
        let decrypted = String::from_utf8(output.stdout)?;

        Ok(toml::from_str::<KeyringCredentials>(&decrypted)?)
    }

    fn save(&self, secretfile: &Path, recipient: &str) -> Result<()> {
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
}
