use std::path::PathBuf;
use dirs_next;
use super::super::models::transaction::Transaction;
use serde_json;


pub fn data_file_path() -> PathBuf {
    let base = dirs_next::data_dir()
        .unwrap_or_else(|| {
            dirs_next::home_dir().expect("Could not find a home directory")
        });

    base.join("fincli").join("transaction.json")
}

pub fn load_raw() -> Result<Vec<Transaction>, std::io::Error> {
    let path = data_file_path();
    let json = std::fs::read_to_string(path).unwrap();
    
    if json.trim().is_empty() {
        return Ok(Vec::new());
    }
    
    let data = serde_json::from_str::<Vec<Transaction>>(&json)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(data)
}

pub fn save_raw(contents: &Vec<Transaction>) -> Result<(), std::io::Error> {

    let path = data_file_path();

    if let Some(dir) = path.parent() {
    std::fs::create_dir_all(dir)?;
    }

    Ok(std::fs::write(path, serde_json::to_string(contents).unwrap())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_transaction() -> Transaction {
        Transaction::new(
            1,
            "01/15/2024",
            "25.50",
            "Food",
            Some("Lunch".to_string()),
        ).unwrap()
    }

    #[test]
    fn test_data_file_path() {
        let path = data_file_path();
        assert!(path.is_absolute());
        assert_eq!(
            path.file_name().and_then(|n| n.to_str()),
            Some("transaction.json")
        );
        assert!(path.to_string_lossy().contains("fincli"));
    }

    #[test]
    fn test_save_and_load_transactions() {
        // Create test transactions
        let transactions = vec![
            create_test_transaction(),
            Transaction::new(
                2,
                "01/16/2024",
                "45.00",
                "Transport",
                Some("Gas".to_string()),
            ).unwrap(),
        ];

        // Save transactions
        let save_result = save_raw(&transactions);
        assert!(save_result.is_ok());

        // Load transactions
        let load_result = load_raw();
        assert!(load_result.is_ok());
        
        let loaded_transactions = load_result.unwrap();
        assert_eq!(loaded_transactions.len(), 2);
    }

    #[test]
    fn test_load_empty_file() {
        // This test would require mocking the file system
        // For now, we'll test the logic with an empty string
        let empty_json = "";
        let result: Result<Vec<Transaction>, serde_json::Error> = serde_json::from_str(empty_json);
        assert!(result.is_err()); // Empty string is not valid JSON
    }

    #[test]
    fn test_save_empty_vector() {
        let empty_transactions: Vec<Transaction> = vec![];
        let result = save_raw(&empty_transactions);
        assert!(result.is_ok());
    }

    #[test]
    fn test_transaction_serialization() {
        let transaction = create_test_transaction();
        let json = serde_json::to_string(&transaction).unwrap();
        let _deserialized: Transaction = serde_json::from_str(&json).unwrap();
        
        // Since fields are private, we can only test that serialization/deserialization works
        // without errors
        assert!(json.contains("25.5"));
        assert!(json.contains("Food"));
    }
}
