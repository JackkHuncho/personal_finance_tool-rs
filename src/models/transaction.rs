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
        let (date, amount, category) = parse_trans(raw_date, raw_amount, raw_cat).unwrap();
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
