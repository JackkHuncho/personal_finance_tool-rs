mod analysis;
mod cli;
mod models;
mod storage;
mod utils;


use clap::Parser;
use crate::cli::FinCli;
use crate::cli::commands::Commands;
use crate::storage::file_handler;
use crate::models::transaction;
use crate::utils::validation;
use prettytable;

fn main() {
    // on init check if there is a data file, if not create one.
    validation::data_file_exists();

    let cli = FinCli::parse();

    let mut transactions = file_handler::load_raw().unwrap();

    match cli.command {
        Commands::Add { date, amount, category, note } => {
            transactions.push(transaction::Transaction::new(transactions.len() as u32 + 1, &date, &amount, &category, note).unwrap());
            file_handler::save_raw(&transactions);
        },
        Commands::List {} => {
           transaction::print_transactions(&transactions);
        },
    }
}
