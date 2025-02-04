use serde_json;
use std::fs;
use serde::de::DeserializeOwned;
use std::process::Command;
use inquire::Select;
use inquire::error::InquireError;

pub fn get_json_content<T: DeserializeOwned>(file_path : &str) -> T {
    let content = fs::read_to_string(file_path).expect(&format!("Erreur lors de la lecture du fichier {}", file_path));
    serde_json::from_str(&content).expect(&format!("Erreur lors de la désérialisation du fichier {}", file_path))
}

pub fn execute_command(exe: &str, args: &[&str]) {
    println!("*\n*\n*\nExecuting command: {} {:?}\n*\n*\n*", exe, args);
    Command::new(exe)
        .args(args)
        .status()
        .expect("failed to start external executable");

    println!("*\n*\n*\nCommand executed.\n*\n*\n*");
}
// TODO : Faire du mutliselect sur les targets
pub fn select_choice<'a>(question : &'a str, possible_choices : Vec<&'a str>) -> &'a str {
    let target_ans: Result<&str, InquireError> = Select::new(question, possible_choices).prompt();

    match target_ans {
        Ok(choice) => return choice,
        Err(_) => panic!("Error while selecting target."),
    }
}