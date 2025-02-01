use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage: {} <project_basepath> <command_to_launch>", args[0]);
    }

    let project_basepath = &args[1];
    if !fs::exists(project_basepath).expect("") {
        panic!("Le projet Unreal Engine {} n'existe pas", project_basepath);
    }
    
    // Ca me parait un peu degueu de faire comme ca mais bon
    let command_to_launch = &args[2];
    match command_to_launch.as_str() {
        _ => {
            println!("Commande {} inconnue.\n", command_to_launch);
        }
    }
}