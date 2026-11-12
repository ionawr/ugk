use clap::Parser;
use ugk::cli::*;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(args) => {
            println!("add called with args: {args:?}");
            todo!();
        }
        Commands::Init(args) => {
            println!("init called with args: {args:?}");
            todo!();
        }
    }
}
