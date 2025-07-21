use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct FinCli {

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {

    Add {

        date: String,
        amount: String,
        category: String,
        #[arg(short, long)]
        note: Option<String>,
    },
    List {
    },

}
