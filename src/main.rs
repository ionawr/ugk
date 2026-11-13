use clap::Parser;
use std::path::Path;
use ugk::cli::*;
use ugk::credentials::KeyringCredentials;

fn main() {
    let cli = Cli::parse();
    let mut keyring_credentials = KeyringCredentials::new();

    match &cli.command {
        Commands::Add(args) => {
            println!("add called with args: {args:?}");
            todo!();
        }
        Commands::Init(args) => {
            let secretfile = Path::new(&args.secretfile);

            let _ = keyring_credentials.init(secretfile);
        }
    }
}
