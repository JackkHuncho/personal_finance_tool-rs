mod analysis;
mod cli;
mod models;
mod storage;
mod utils;


use clap::Parser;
use crate::cli::FinCli;
use crate::cli::commands::Commands;
use crate::storage::file_handler;
use crate::models::transaction::Transaction;

fn main() {
    let cli = FinCli::parse();

    println!("Data file path: {:?}", file_handler::data_file_path());

    match cli.command {
        Commands::Add { date, amount, category, note } => {
            let mut transactions = file_handler::load_raw().unwrap();
            transactions.push(Transaction::new(transactions.len() as u32 + 1, &date, &amount, &category, note).unwrap());
        },
        Commands::List {} => {
            () // list logic here
        },
    }
}
