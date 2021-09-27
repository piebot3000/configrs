use clap::{Arg, App, SubCommand};
use confy;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::collections::HashMap;
use std::path::PathBuf;

const VERSION: &str = "0.1.0";

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    version: String,
    repo: HashMap<String, PathBuf>,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            version: VERSION.to_string(),
            repo: HashMap::default(),
        }
    }
}

fn run() -> Result<(), Box<dyn Error>>{
    // setting up arguments and whatnot
    let args = App::new("configrs")
        .version(VERSION)
        .author("Sage Wynn")
        .about("Helps keep track of config files.")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .help("sets a custom config file directory")
            .takes_value(true))
        .subcommand(SubCommand::with_name("edit")
            .arg(Arg::with_name("file")
                .help("which file to be edited")
                .required(false)
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
        .get_matches();

    // loading either from confys default, or a user supplied config
    let mut cfg: Config = load_cfg(args.value_of("config"))?;

    // match which subcommand has been used
    match args.subcommand() {
        ("edit", Some(edit_args)) => {
            edit(
                &mut cfg, 
                edit_args.value_of("file")
                    .expect("file somehow missing, clap.")
            )?;
        },
        ("add", Some(add_args)) => {
            add(
                &mut cfg, 
                add_args.value_of("name").expect("name somehow missing, clap."), 
                add_args.value_of("file").expect("file somehow missing, clap.")
            );
            store_cfg(args.value_of("config"), cfg)?;
        },
        _ => unreachable!()
    }


    // everything went ok
    Ok(())
}

fn add(cfg: &mut Config, name: &str, file: &str) {
    cfg.repo.insert(name.to_string(), PathBuf::from(file));
}

fn edit(cfg: &mut Config, name: &str) -> Result<(), String> {
    let path = match cfg.repo.get(name) {
        Some(p) => p,
        None => return Err("That file is not in the repository.".to_string())
    };

    let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());
    match std::process::Command::new(editor)
        .arg(path)
        .status() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to run editor: {:?}", e))
    }
}

fn load_cfg(path: Option<&str>) -> Result<Config, confy::ConfyError> {
    // loading either from confys default, or a user supplied config
    match path {
        Some(dir) => confy::load_path(&dir),
        None => confy::load("configrs")
    }
}

fn store_cfg(path: Option<&str>, cfg: Config) -> Result<(), confy::ConfyError> {
    // saving to either confys default or a user supplied default
    match path {
        Some(dir) => confy::store_path(&dir, cfg),
        None => confy::store("configrs", cfg)
    }
}
    
fn main() {
    let result = run();

    match result {
        Ok(()) => {
        },
        Err(e) => {
            eprintln!("error {:?}", e);
        }
    }

}
