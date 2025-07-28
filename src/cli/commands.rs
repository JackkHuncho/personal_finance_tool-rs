use crate::models::transaction;
use crate::storage::file_handler;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add {
        date: String,
        amount: String,
        category: String,
        #[arg(short, long)]
        note: Option<String>,
    },
    List {},
}

impl Commands {
    pub fn process(&self) {
        let mut transactions = file_handler::load_raw().unwrap();

        match self {
            Commands::Add {
                date,
                amount,
                category,
                note,
            } => {
                transactions.push(
                    transaction::Transaction::new(
                        transactions.len() as u32 + 1,
                        &date,
                        &amount,
                        &category,
                        Some(note.clone().unwrap_or("N/A".to_string())),
                    )
                    .expect("idk"),
                );
                let _ = file_handler::save_raw(&transactions);
            }
            Commands::List {} => {
                transaction::print_transactions(&transactions);
            }
        }
    }
}
