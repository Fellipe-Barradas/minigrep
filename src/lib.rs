use clap::Parser;
use std::process;
use commands::CommandsEnum;

pub mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Command to run
    #[command(subcommand)]
    pub command: Option<CommandsEnum>,
}

use commands::{count, find_word};
pub fn run(){
    let args = Cli::parse();

    match &args.command {
        Some(CommandsEnum::Count{input, file}) => {
            count(input, file).unwrap_or_else(|err|{
                println!("Erro ao tentar abrir o arquivo, {}", err.to_string());
                process::exit(1)
            });
        },
        Some(CommandsEnum::Find { input, file }) => {
            find_word(input, file).unwrap_or_else(|err|{
                println!("Erro ao tentar abrir o arquivo, {}", err.to_string());
                process::exit(1)
            });
        }
        None => {
            println!("No command");
        }

    }
}