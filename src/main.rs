use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List { path: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List { path } => commands::cmd_list(path),
    }
}
