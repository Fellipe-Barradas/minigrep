use clap::Parser;

mod commands;

use commands::CommandsEnum;

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
           println!("Counting => input: {}, file: {}", input, file);
        },
        Some(CommandsEnum::Find { input, file }) => {
            println!("Finding => input: {}, file: {}", input, file)
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