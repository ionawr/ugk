use anyhow::Result;
use clap::Parser;
use rpassword::prompt_password;
use std::path::PathBuf;
use ugk::cli::*;
use ugk::config::*;
use ugk::credentials::KeyringCredentials;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let secretfile = cli
        .secretfile
        .map_or_else(|| default_secretfile_path(), |v| Ok(PathBuf::from(v)))?;
    let mut keyring_credentials = KeyringCredentials::load(&secretfile).unwrap_or_else(|_| {
        dbg!("load failed");
        KeyringCredentials::new()
    });

    match &cli.command {
        Commands::Init(_args) => {
            keyring_credentials.init(&secretfile)?;
            println!(
                "Successfully initialised secretfile at {}",
                &secretfile.display()
            );
            Ok(())
        }
        Commands::Add(args) => {
            let password = prompt_password("Password to the keyring: ")?;
            keyring_credentials.add(&args.keyring_name, &password)?;

            keyring_credentials.save(&secretfile, &args.recipient)?;

            println!("Successfully added to secretfile.");
            Ok(())
        }
        Commands::Remove(args) => {
            keyring_credentials.remove(&args.keyring_name)?;
            keyring_credentials.save(secretfile, &args.recipient)?;
            Ok(())
        }
    }
}
