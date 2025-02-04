use std::{env, fs};
pub mod types;
use regex::Regex;
pub mod utils;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Utilisation: {} <chemin du fichier uproject> <commande à lancer>", args[0]);
    }

    let project_basepath = &args[1];
    if !fs::exists(project_basepath).expect("") {
        panic!("Le projet Unreal Engine {} n'existe pas", project_basepath);
    }
    
    // Ca me parait un peu degueu de faire comme ca mais bon
    let command_to_launch = &args[2];
    match command_to_launch.as_str() {
        "show-infos" => { show_infos(project_basepath); },
        "build" => { build(project_basepath); },
        "package" => { package(project_basepath); },
        _ => {
            println!("Commande {} inconnue.\nLa commande possible est :", command_to_launch);
            println!("\tshow-infos : Montre les informations du projet.");
            println!("\tbuild : Compile le projet.");
            println!("\tpackage <chemin du package> : Package le projet.");
        }
    }
}

fn show_infos(project_basepath : &str) {
    // TODO : Recup nom du projet depuis uproject s'il est définit
    let project_name = project_basepath.split('\\').last().unwrap().split(".uproject").next().unwrap();
    let uproject_json_content : types::UProject = utils::get_json_content(project_basepath);

    let re = Regex::new(r"^\d+\.\d+$").unwrap();
    let engine_version;
    if re.is_match(&uproject_json_content.EngineAssociation) {
        engine_version = uproject_json_content.EngineAssociation;
    } else {
        // TODO : Remonter les dossiers pour trouver le Unreal source plutôt que de partir du dossier courant
        let build_version_basepath = "./Engine/Build/Build.version";
        if !fs::exists(build_version_basepath).expect("") {
            panic!("Impossible de récupérer le fichier build version");
        }

        let content_json : serde_json::Value = utils::get_json_content(build_version_basepath);
        engine_version = format!("{}.{}.{} (From Source)", content_json["MajorVersion"], content_json["MinorVersion"], content_json["PatchVersion"]);
    }
    
    // Affichage
    println!("Nom du projet : {}", project_name);
    println!("Version du moteur : {}", engine_version);
    if let Some(plugins) = uproject_json_content.Plugins {
        println!("Plugins :");
        for plugin in plugins {
            println!("\t・{} : {}", plugin.Name, (if plugin.Enabled {"Activé"} else {"Désactivé"}));
        }
    } else {
        println!("Aucun plugin dans le projet.");
    }

    return;
}


fn build(project_basepath : &str) {
    let project_name = project_basepath.split('\\').last().unwrap().split(".uproject").next().unwrap();

    let target = utils::select_choice("What's your target platform ?", vec!["Win64", "Mac", "IOS", "Android", "Linux", "LinuxArm64", "TVOS", "VisionOS"]);
    //let opti = utils::select_choice("What's your optimization ?", vec!["Development", "Production"]);

    utils::execute_command("./Engine/Build/BatchFiles/Build.bat", &[project_name, target, "Development", project_basepath, "-waitmutex"]);
    return;
}

fn package(project_basepath : &str) {
    /* Command line example
    C:/Users/Romain/Documents/UnrealEngine/Games/MyTestProject/MyTestProject.uproject BuildCookRun
    -project=C:/Users/Romain/Documents/UnrealEngine/Games/MyTestProject/MyTestProject.uproject
    -noP4 -clientconfig=Development -serverconfig=Development -nocompileeditor
    -unrealexe=C:\Users\Romain\Documents\UnrealEngine\Engine\Binaries\Win64\UnrealEditor-Cmd.exe
    -utf8output -platform=Win64 -build -cook -map=ThirdPersonMap+ThirdPersonMap -CookCultures=en
    -unversionedcookedcontent -stage -package -cmdline="ThirdPersonMap -Messaging"
    -addcmdline="-SessionId=4A12A4C64656F57DD62E68A13C16AD3D -SessionOwner='Romain' -SessionName='UnNomInteressant'   "
    */

    // C:/Users/Romain/Documents/UnrealEngine/Games/MyTestProject/MyTestProject.uproject BuildCookRun -project=C:/Users/Romain/Documents/UnrealEngine/Games/MyTestProject/MyTestProject.uproject -noP4 -clientconfig=Development -serverconfig=Development -nocompileeditor -unrealexe=C:\Users\Romain\Documents\UnrealEngine\Engine\Binaries\Win64\UnrealEditor-Cmd.exe -utf8output -platform=Win64 -build -cook -map=ThirdPersonMap+ThirdPersonMap -CookCultures=en -unversionedcookedcontent -stage -package -cmdline="ThirdPersonMap -Messaging" -addcmdline="-SessionId=4A12A4C64656F57DD62E68A13C16AD3D -SessionOwner='Romain' -SessionName='UnNomInteressant'   "
    // Parsing command line: -ScriptsForProject=C:/Users/Romain/Documents/UnrealEngine/Games/MyTestProject/MyTestProject.uproject BuildCookRun -project=C:/Users/Romain/Documents/UnrealEngine/Games/MyTestProject/MyTestProject.uproject -noP4 -clientconfig=Development -serverconfig=Development -nocompileeditor -unrealexe=C:\Users\Romain\Documents\UnrealEngine\Engine\Binaries\Win64\UnrealEditor-Cmd.exe -utf8output -platform=Win64 -build -cook -map=ThirdPersonMap+ThirdPersonMap -CookCultures=en -unversionedcookedcontent -stage -package -cmdline="ThirdPersonMap -Messaging" -addcmdline="-SessionId=4A12A4C64656F57DD62E68A13C16AD3D -SessionOwner='Romain' -SessionName='UnNomInteressant'   "

    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        panic!("Utilisation: {} <chemin du fichier uproject> package <chemin du package>", args[0]);
    }
    let package_path = &args[3];

    println!("On package {} vers {}", project_basepath, package_path);

    utils::execute_command("./Engine/Build/BatchFiles/RunUAT.bat", &[
        "BuildCookRun",
        &format!("-project={}", project_basepath),
        "-noP4",
        "-clientconfig=Development",
        "-serverconfig=Development",
        "-nocompileeditor",
        "-unrealexe=./Engine/Binaries/Win64/UnrealEditor-Cmd.exe",
        "-utf8output",
        "-platform=Win64",
        "-build",
        "-cook",
        "-map=ThirdPersonMap+ThirdPersonMap", // Pas bon
        "-CookCultures=en",
        "-unversionedcookedcontent",
        "-stage",
        "-package",
        "-cmdline=\"ThirdPersonMap -Messaging\"", // Pas bon
        "-addcmdline=\"-SessionId=4A12A4C64656F57DD62E68A13C16AD3D -SessionOwner='Romain' -SessionName='UnNomInteressant'\"", // Pas bon
    ]);
}