use std::path::PathBuf;
use dirs_next;
use std::{io,fs};
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
