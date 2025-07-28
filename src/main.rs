mod analysis;
mod cli;
mod models;
mod storage;
mod utils;

use clap::Parser;
use crate::cli::FinCli;
use crate::utils::validation;

fn main() {
    // on init check if there is a data file, if not create one.
    validation::data_file_exists();

    let cli = FinCli::parse();
    cli.command.process();

}
