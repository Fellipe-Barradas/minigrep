use std::process;

use clap::Parser;

mod commands;

use commands::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[command(subcommand)]
    command: Option<CommandsEnum>,
}

fn main() {
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

#[test]
fn verify_cli(){
    use clap::CommandFactory;
    Cli::command().debug_assert();
}