use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand)]
enum CliCommands {
    /// Configures this tool
    Config,
    /// Performs backup
    Backup,
}

fn main() {
    let args = CliArgs::parse();

    match args.command {
        CliCommands::Config => {
            println!("Config!");
        }
        CliCommands::Backup => {
            println!("Backup!");
        }
    };
}
