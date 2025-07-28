mod analysis;
mod cli;
mod models;
mod storage;
mod utils;

use crate::cli::FinCli;
use crate::utils::validation;
use clap::Parser;

fn main() {
    // on init check if there is a data file, if not create one.
    validation::data_file_exists();

    let cli = FinCli::parse();
    cli.command.process();
}
