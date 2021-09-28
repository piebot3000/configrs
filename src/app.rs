use clap::{Arg, App, SubCommand, AppSettings};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub repo: HashMap<String, String>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            repo: HashMap::default(),
        }
    }
}

pub fn build() -> App<'static, 'static> {
    App::new("configrs")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("0.1.0")
        .author("Sage Wynn")
        .about("Helps keep track of config files.")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .help("sets a custom config file")
            .takes_value(true))
        .subcommand(SubCommand::with_name("edit")
            .about("edits a file in the repository")
            .arg(Arg::with_name("file")
                .help("which file to be edited")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("add")
            .about("adds a new file to the repository")
            .arg(Arg::with_name("name")
                 .help("name for the file")
                 .required(true)
                 .index(1))
            .arg(Arg::with_name("file")
                 .help("path of the config file")
                 .required(true)
                 .index(2)))
        .subcommand(SubCommand::with_name("remove")
            .about("removes a file from the repository")
            .arg(Arg::with_name("file")
                .help("file to be removed")
                .required(true)
                .index(1)))
}


pub fn load_cfg(path: Option<&str>) -> Result<Config, confy::ConfyError> {
    // loading either from confys default, or a user supplied config
    match path {
        Some(dir) => confy::load_path(&dir),
        None => confy::load("configrs")
    }
}

pub fn store_cfg(path: Option<&str>, cfg: Config) -> Result<(), confy::ConfyError> {
    // saving to either confys default or a user supplied default
    match path {
        Some(dir) => confy::store_path(&dir, cfg),
        None => confy::store("configrs", cfg)
    }
}
