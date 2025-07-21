mod analysis;
mod cli;
mod models;
mod storage;
mod utils;

fn main() {
    let cli = cli::commands::FinCli::parse();
    println!("{:?}", cli);
}
