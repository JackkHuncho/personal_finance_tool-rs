use super::super::models::transaction::Transaction;
use dirs_next;
use serde_json;
use std::path::PathBuf;

pub fn data_file_path() -> PathBuf {
    let base = dirs_next::data_dir()
        .unwrap_or_else(|| dirs_next::home_dir().expect("Could not find a home directory"));

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

    Ok(std::fs::write(
        path,
        serde_json::to_string(contents).unwrap(),
    )?)
}

// need to add edge case testing, currently there is only functional testing
/*
#[cfg(test)]
mod tests{

    use super::*;

    const trans:Transaction = Transaction::new(1 , "11/30/2003", 16.00, "Food", "yummy");
    const trans_json:&str = "";
    #[test]
    fn serialize() {
        assert_eq!(trans_json,serde_json::to_string(&trans));
    }

    #[test]
    fn deserialize() {
        assert_eq!(trans, serde_json::from_str(&trans_json));
    }

    #[test]
    fn save() {
        assert!(save_raw(&trans));
    }

    #[test]
    fn load() {
        assert!(load_raw());
    }
}
*/
