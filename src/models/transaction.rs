use super::category::Category;
use crate::storage::file_handler;
use chrono::NaiveDate;
use prettytable::{row, Cell, Row, Table};
use serde;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Transaction {
    id: u32,
    date: chrono::NaiveDate,
    amount: f64,
    category: Category,
    note: Option<String>,
}

#[derive(Debug)]
pub enum TransactionErr {
    DateParse,
    AmountParse,
    CategoryParse,
}

impl Transaction {
    pub fn new(
        id: u32,
        raw_date: &str,
        raw_amount: &str,
        raw_cat: &str,
        note: Option<String>,
    ) -> Result<Self, TransactionErr> {
        let (date, amount, category) = parse_trans(raw_date, raw_amount, raw_cat)?;
        Ok(Transaction {
            id,
            date,
            amount,
            category,
            note,
        })
    }

    pub fn add(
        raw_date: &str,
        raw_amount: &str,
        raw_cat: &str,
        note: Option<String>,
    ) -> Result<(), TransactionErr> {
        let mut all_trans = file_handler::load_raw().unwrap();
        let new_trans = Self::new(
            all_trans.len() as u32 + 1,
            raw_date,
            raw_amount,
            raw_cat,
            note,
        )
        .unwrap();

        all_trans.push(new_trans);
        let _ = file_handler::save_raw(&all_trans);
        Ok(())
    }
}

fn parse_trans(
    raw_date: &str,
    raw_amount: &str,
    raw_cat: &str,
) -> Result<(NaiveDate, f64, Category), TransactionErr> {
    let date =
        NaiveDate::parse_from_str(raw_date, "%m/%d/%Y").map_err(|_| TransactionErr::DateParse)?;
    let amount = raw_amount
        .parse::<f64>()
        .map_err(|_| TransactionErr::AmountParse)?;
    let category = raw_cat
        .parse::<Category>()
        .map_err(|_| TransactionErr::CategoryParse)?;

    Ok((date, amount, category))
}

pub fn print_transactions(transactions: &Vec<Transaction>) {
    let mut table = Table::new();

    // Add header row
    table.add_row(row!["ID", "Amount", "Category", "Date", "Notes"]);

    // Add data rows
    for transaction in transactions {
        table.add_row(Row::new(vec![
            Cell::new(&transaction.id.to_string()),
            Cell::new(&format!("${:.2}", transaction.amount)),
            Cell::new(&transaction.category.to_string()),
            Cell::new(&transaction.date.to_string()),
            Cell::new(&transaction.note.as_ref().unwrap_or(&String::from("N/A"))),
        ]));
    }
    table.printstd();
}


#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn test_trans_parse_happy() {
        // arrange
        let raw_date:&str = "11/30/2000";
        let raw_amount:&str = "15.00";
        let raw_cat:&str = "Food";
        let parse = parse_trans(raw_date, raw_amount, raw_cat);

        let date = NaiveDate::from_ymd_opt(2000,11,30).unwrap();
        let amount:f64 = 15.00;
        let cat = Category::Food;
        
        assert_eq!((date, amount, cat), parse.unwrap());
    }

    #[test]
    fn test_transaction_new_success() {
        let transaction = Transaction::new(
            1,
            "12/25/2024",
            "100.50",
            "Entertainment",
            Some("Christmas gift".to_string()),
        ).unwrap();
        
        // Since fields are private, we can only test that creation succeeds
        // and the transaction can be serialized
        let json = serde_json::to_string(&transaction).unwrap();
        assert!(json.contains("100.5"));
        assert!(json.contains("Entertainment"));
    }

    #[test]
    fn test_transaction_new_without_note() {
        let transaction = Transaction::new(
            2,
            "01/01/2024",
            "25.00",
            "Transport",
            None,
        ).unwrap();
        
        let json = serde_json::to_string(&transaction).unwrap();
        assert!(json.contains("25"));
        assert!(json.contains("Transport"));
    }

    #[test]
    fn test_transaction_new_invalid_date() {
        let result = Transaction::new(
            3,
            "invalid-date",
            "50.00",
            "Food",
            None,
        );
        
        assert!(result.is_err());
        match result {
            Err(TransactionErr::DateParse) => (),
            _ => panic!("Expected DateParse error"),
        }
    }

    #[test]
    fn test_transaction_new_invalid_amount() {
        let result = Transaction::new(
            4,
            "01/01/2024",
            "not-a-number",
            "Food",
            None,
        );
        
        assert!(result.is_err());
        match result {
            Err(TransactionErr::AmountParse) => (),
            _ => panic!("Expected AmountParse error"),
        }
    }

    #[test]
    fn test_transaction_new_invalid_category() {
        let result = Transaction::new(
            5,
            "01/01/2024",
            "50.00",
            "InvalidCategory",
            None,
        );
        
        assert!(result.is_err());
        match result {
            Err(TransactionErr::CategoryParse) => (),
            _ => panic!("Expected CategoryParse error"),
        }
    }

    #[test]
    fn test_parse_trans_invalid_date() {
        let result = parse_trans("invalid", "50.00", "Food");
        assert!(result.is_err());
        match result {
            Err(TransactionErr::DateParse) => (),
            _ => panic!("Expected DateParse error"),
        }
    }

    #[test]
    fn test_parse_trans_invalid_amount() {
        let result = parse_trans("01/01/2024", "invalid", "Food");
        assert!(result.is_err());
        match result {
            Err(TransactionErr::AmountParse) => (),
            _ => panic!("Expected AmountParse error"),
        }
    }

    #[test]
    fn test_parse_trans_invalid_category() {
        let result = parse_trans("01/01/2024", "50.00", "InvalidCategory");
        assert!(result.is_err());
        match result {
            Err(TransactionErr::CategoryParse) => (),
            _ => panic!("Expected CategoryParse error"),
        }
    }

    #[test]
    fn test_transaction_serialization_roundtrip() {
        let original = Transaction::new(
            1,
            "12/25/2024",
            "100.50",
            "Entertainment",
            Some("Christmas gift".to_string()),
        ).unwrap();
        
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Transaction = serde_json::from_str(&json).unwrap();
        
        // Test that serialization/deserialization works without errors
        let json2 = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(json, json2);
    }

    #[test]
    fn test_parse_trans_various_categories() {
        let categories = vec![
            ("Food", Category::Food),
            ("Transport", Category::Transport),
            ("Entertainment", Category::Entertainment),
            ("Shopping", Category::Shopping),
            ("Bills", Category::Bills),
            ("Healthcare", Category::Healthcare),
            ("Automotive", Category::Automotive),
            ("Salary", Category::Salary),
            ("Investment", Category::Investment),
            ("Freelance", Category::Freelance),
        ];

        for (input, expected) in categories {
            let result = parse_trans("01/01/2024", "50.00", input);
            assert!(result.is_ok());
            let (_, _, category) = result.unwrap();
            assert_eq!(category, expected);
        }
    }

    #[test]
    fn test_parse_trans_custom_category() {
        let result = parse_trans("01/01/2024", "50.00", "income:Bonus");
        assert!(result.is_ok());
        let (_, _, category) = result.unwrap();
        
        match category {
            Category::Custom { income_or_expense, name } => {
                assert_eq!(income_or_expense, "Income");
                assert_eq!(name, "Bonus");
            }
            _ => panic!("Expected Custom category"),
        }
    }
}
