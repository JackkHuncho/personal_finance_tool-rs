use crate::storage::file_handler;

pub fn data_file_exists() {
    let path = file_handler::data_file_path();

    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
        if !path.exists() {
            let _ = std::fs::File::create(path);
        }
    }
}
