mod backuper;
mod client;
mod config;
mod config_repository;
mod mj_domain;

use crate::backuper::backup;
use crate::client::MjClient;
use crate::config_repository::ConfigRepository;
use clap::{Parser, Subcommand};
use home::home_dir;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    command: CliCommands,
    /// Optional account name
    #[arg(global(true))]
    account: Option<String>,
}

impl CliArgs {
    fn get_account_name(&self) -> &str {
        if let None = self.account {
            return "default";
        }

        self.account.as_ref().unwrap().as_str()
    }
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
            let config = ConfigRepository::read();
            let account_name = args.get_account_name();
            let account = config.config.get_account(account_name);

            if let None = account {
                println!("Account {} not found", account_name);

                return;
            }

            let client = MjClient::new(account.unwrap());

            backup(&client, get_backup_path(account_name));
        }
    };
}

fn get_backup_path(account_name: &str) -> PathBuf {
    let mut path = home_dir().expect("Could not detect your home directory");
    path.push("mj-tpl-backups");
    path.push(account_name);

    if path.exists() {
        return path;
    }

    if let Err(_) = fs::create_dir_all(path.clone()) {
        panic!("Could not create directory {}", path.to_str().unwrap());
    }

    path
}
