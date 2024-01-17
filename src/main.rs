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
            count(input, file);
        },
        Some(CommandsEnum::Find { input, file }) => {
            find_word(input, file);
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