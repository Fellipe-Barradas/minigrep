use clap::Subcommand;


#[derive(Subcommand)]
pub enum CommandsEnum {
    Count {
        /// Word to count
        input: String,

        /// File to read from
        file: String,
    },
    Find {
        /// Word to find
        input: String,

        /// File to read from
        file: String,
    },
}