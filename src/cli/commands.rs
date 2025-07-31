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
                    crate::models::transaction::Transaction::new(
                        transactions.len() as u32 + 1,
                        &date,
                        &amount,
                        &category,
                        Some(note.clone().unwrap_or("N/A".to_string())),
                    )
                    .expect("Issue with creating new transaction"),
                );

                match file_handler::save_raw(&transactions) {
                    Ok(_) => (),
                    Err(e) => eprintln!("{}",e),
                }
            }
            Commands::List {} => {
                crate::models::transaction::print_transactions(&transactions);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_command_creation() {
        let command = Commands::Add {
            date: "01/15/2024".to_string(),
            amount: "25.50".to_string(),
            category: "Food".to_string(),
            note: Some("Lunch".to_string()),
        };

        match command {
            Commands::Add { date, amount, category, note } => {
                assert_eq!(date, "01/15/2024");
                assert_eq!(amount, "25.50");
                assert_eq!(category, "Food");
                assert_eq!(note, Some("Lunch".to_string()));
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_add_command_without_note() {
        let command = Commands::Add {
            date: "01/15/2024".to_string(),
            amount: "25.50".to_string(),
            category: "Food".to_string(),
            note: None,
        };

        match command {
            Commands::Add { date, amount, category, note } => {
                assert_eq!(date, "01/15/2024");
                assert_eq!(amount, "25.50");
                assert_eq!(category, "Food");
                assert_eq!(note, None);
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_list_command() {
        let command = Commands::List {};
        match command {
            Commands::List {} => {
                // List command has no fields to test
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_transaction_creation_from_add_command() {
        let add_command = Commands::Add {
            date: "01/15/2024".to_string(),
            amount: "25.50".to_string(),
            category: "Food".to_string(),
            note: Some("Lunch".to_string()),
        };

        match add_command {
            Commands::Add { date, amount, category, note } => {
                let transaction = crate::models::transaction::Transaction::new(
                    1,
                    &date,
                    &amount,
                    &category,
                    note,
                ).unwrap();

                // Test that the transaction was created successfully
                let json = serde_json::to_string(&transaction).unwrap();
                assert!(json.contains("25.5"));
                assert!(json.contains("Food"));
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_transaction_creation_with_invalid_data() {
        let add_command = Commands::Add {
            date: "invalid-date".to_string(),
            amount: "25.50".to_string(),
            category: "Food".to_string(),
            note: None,
        };

        match add_command {
            Commands::Add { date, amount, category, note } => {
                let result = crate::models::transaction::Transaction::new(
                    1,
                    &date,
                    &amount,
                    &category,
                    note,
                );

                assert!(result.is_err());
            }
            _ => panic!("Expected Add command"),
        }
    }

    #[test]
    fn test_id_increment_logic() {
        // Test that IDs increment correctly
        let transactions = vec![
            crate::models::transaction::Transaction::new(1, "01/15/2024", "25.50", "Food", None).unwrap(),
            crate::models::transaction::Transaction::new(2, "01/16/2024", "30.00", "Transport", None).unwrap(),
        ];

        let next_id = transactions.len() as u32 + 1;
        assert_eq!(next_id, 3);

        let new_transaction = crate::models::transaction::Transaction::new(
            next_id,
            "01/17/2024",
            "15.00",
            "Entertainment",
            None,
        ).unwrap();

        // Verify the transaction was created with the correct ID
        let json = serde_json::to_string(&new_transaction).unwrap();
        // Note: We can't directly test the ID since it's private, but we can verify
        // the transaction was created successfully
        assert!(json.contains("15"));
        assert!(json.contains("Entertainment"));
    }
}
