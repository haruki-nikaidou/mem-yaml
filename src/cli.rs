use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "mem-yaml")]
#[command(about = "Anki's alternative for who hates GUI and mouse clicks")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init(DirArgs),
    Start(DirArgs),
}

#[derive(Debug, Args)]
pub struct DirArgs {
    #[arg(short, long, default_value = ".")]
    pub dir: String
}