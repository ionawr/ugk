use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long, env = "UGK_SECRETFILE_PATH")]
    pub secretfile: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Add(AddArgs),
    Init(InitArgs),
    #[command(name = "rm")]
    Remove(RemoveArgs),
}

#[derive(Args, Debug)]
pub struct AddArgs {
    #[arg(short, long)]
    pub recipient: String,
    #[arg(short, long)]
    pub keyring_name: String,
}

#[derive(Args, Debug)]
pub struct InitArgs {}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    #[arg(short, long)]
    pub recipient: String,
    #[arg(short, long)]
    pub keyring_name: String,
}
