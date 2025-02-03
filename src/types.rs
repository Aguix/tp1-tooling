use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct UProject {
    pub EngineAssociation: String,
    pub Modules: Option<Vec<Module>>,
    pub Plugins: Option<Vec<Plugin>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Module {
    pub Name: String,
    pub Type: String,
    pub LoadingPhase: String,
    pub AdditionalDependencies: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct Plugin {
    pub Name: String,
    pub Enabled: bool,
}