use super::category::Category;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    id: u32,
    date: chrono::DateTime<Utc>,
    amount: f64,
    category: Category,
    note: Option<String>,
}

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
        let date = DateTime::parse_from_rfc3339(raw_date)
            .map_err(|_| TransactionErr::DateParse)?
            .with_timezone(&Utc);
        let amount = raw_amount
            .parse::<f64>()
            .map_err(|_| TransactionErr::AmountParse)?;
        let category = raw_cat
            .parse::<Category>()
            .map_err(|_| TransactionErr::CategoryParse)?;
        Ok(Transaction {
            id,
            date,
            amount,
            category,
            note,
        })
    }
}
