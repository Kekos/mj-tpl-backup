use super::CliArgs;
use crate::config_repository::ConfigRepository;
use serde_derive::{Deserialize, Serialize};
use std::io::stdin;
use std::string::String;

#[derive(Serialize, Deserialize)]
pub struct Config {
    accounts: Vec<MjAccount>,
}

#[derive(Serialize, Deserialize)]
pub struct MjAccount {
    name: String,
    private: String,
    public: String,
}

impl Config {
    pub fn get_account(&self, name: &str) -> Option<&MjAccount> {
        self.accounts.iter().find(|x| x.name == name)
    }

    pub fn get_account_mut(&mut self, name: &str) -> Option<&mut MjAccount> {
        self.accounts.iter_mut().find(|x| x.name == name)
    }
}

pub fn list_accounts() {
    println!("Name         | Private key            | Public key");
    println!("---------------------------------------------------------------");

    let config_repo = ConfigRepository::read();
    config_repo.config.accounts.iter().for_each(|account| {
        println!(
            "{} | {} | {}",
            account.name, account.private, account.public
        )
    });
}

pub fn new_account() {
    println!("Give the account name or leave blank: ");
    let mut name = try_read_line();
    if name.is_empty() {
        name = String::from("default");
    }

    println!("Enter Mailjet Private key: ");
    let private_key = try_read_line();

    println!("Enter Mailjet Public key: ");
    let public_key = try_read_line();

    let mut config_repo = ConfigRepository::read();

    if let Some(_) = config_repo.config.get_account(&name) {
        println!(
            "An account with name {} is already defined! Try `edit` subcommand instead.",
            name
        );

        return;
    }

    config_repo.config.accounts.push(MjAccount {
        name,
        private: private_key,
        public: public_key,
    });

    ConfigRepository::write(&config_repo);

    todo!("connect");
}

pub fn edit_account(args: &CliArgs) {
    let mut config_repo = ConfigRepository::read();
    let account_name = args.get_account_name();
    let mut account = match config_repo.config.get_account_mut(account_name) {
        None => {
            println!("Account with name {} not found!", account_name);

            return;
        }
        Some(a) => a,
    };

    println!("Enter Mailjet Private key: ");
    let private_key = try_read_line();

    println!("Enter Mailjet Public key: ");
    let public_key = try_read_line();

    account.private = private_key;
    account.public = public_key;

    ConfigRepository::write(&config_repo);

    todo!("connect");
}

pub fn delete_account(args: &CliArgs) {
    let mut config_repo = ConfigRepository::read();
    let account_name = args.get_account_name();

    let old_size = config_repo.config.accounts.len();

    config_repo
        .config
        .accounts
        .retain(|a| a.name != account_name);

    let new_size = config_repo.config.accounts.len();

    if old_size == new_size {
        println!("Account with name {} not found!", account_name);

        return;
    }

    ConfigRepository::write(&config_repo);
}

fn try_read_line() -> String {
    let mut input_string = String::new();

    stdin()
        .read_line(&mut input_string)
        .expect("Could not read line");

    input_string.trim().to_string()
}
