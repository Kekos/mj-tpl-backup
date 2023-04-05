mod config;
mod config_repository;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    command: CliCommands,
    /// Optional account name
    #[arg(global(true))]
    account: Option<String>,
}

#[derive(Subcommand)]
enum CliCommands {
    /// Configures this tool
    Config {
        #[command(subcommand)]
        config_command: CliConfigCommands,
    },
    /// Performs backup
    Backup,
}

#[derive(Subcommand)]
enum CliConfigCommands {
    /// Lists all configured accounts
    List,
    /// Configures an new account
    New,
    /// Edits an existing account
    Edit,
    /// Deletes an account
    Delete,
}

fn main() {
    let args = CliArgs::parse();

    match args.command {
        CliCommands::Config { ref config_command } => match config_command {
            CliConfigCommands::List => config::list_accounts(),
            CliConfigCommands::New => config::new_account(),
            CliConfigCommands::Edit => config::edit_account(&args),
            CliConfigCommands::Delete => config::delete_account(&args),
        },
        CliCommands::Backup => {
            println!("Backup!");
        }
    };
}
