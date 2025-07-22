pub mod commands;
pub mod display;

use super::cli::commands::Commands;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct FinCli {

    #[command(subcommand)]
    pub command: Commands,
}
