use serde_json;
use std::fs;
use serde::de::DeserializeOwned;

pub fn get_json_content<T: DeserializeOwned>(file_path : &str) -> T {
    let content = fs::read_to_string(file_path).expect(&format!("Erreur lors de la lecture du fichier {}", file_path));
    serde_json::from_str(&content).expect(&format!("Erreur lors de la désérialisation du fichier {}", file_path))
}