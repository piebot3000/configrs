use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub repo: HashMap<String, PathBuf>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            repo: HashMap::<String, PathBuf>::default(),
        }
    }
}

#[derive(StructOpt)]
#[structopt(name="Configrs", about="simple utility to help deal with config files")]
pub struct Opt {
    #[structopt(short, long)]
    pub add: bool,
    #[structopt(short, long)]
    pub remove: bool,
    #[structopt(short, long)]
    pub yoink: bool,

    #[structopt()]
    pub name: String,

    #[structopt(short, long="config")]
    pub config_file: Option<PathBuf>,

    #[structopt(required_if("add", "true"), required_if("remove", "true"))]
    pub file: Option<PathBuf>,

    #[structopt(required_if("yoink", "true"))]
    pub directory: Option<PathBuf>,
}

pub fn load_cfg(path: &Option<PathBuf>) -> Result<Config, confy::ConfyError> {
    // loading either from confys default, or a user supplied config
    match path {
        Some(dir) => confy::load_path(&dir),
        None => confy::load("configrs"),
    }
}

pub fn store_cfg(path: &Option<PathBuf>, cfg: Config) -> Result<(), confy::ConfyError> {
    // saving to either confys default or a user supplied default
    match path {
        Some(dir) => confy::store_path(&dir, cfg),
        None => confy::store("configrs", cfg),
    }
}
