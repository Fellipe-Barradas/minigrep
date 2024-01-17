use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Count{
        /// Palavra de busca
        #[arg()]
        input: String,

        /// Arquivo de entrada
        #[arg()]
        file: String,
    },
    Find {
        /// Palavra de busca
        #[arg()]
        input: String,

        /// Arquivo de entrada
        #[arg()]
        file: String,
    }
}
fn main() {
   let args = Cli::parse();

   match &args.command {
        Some(Commands::Count{input, file}) => {
           println!("Counting => input: {}, file: {}", input, file);
        },
        Some(Commands::Find { input, file }) => {
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