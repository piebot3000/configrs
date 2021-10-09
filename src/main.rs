use std::error::Error;
use std::path::PathBuf;
use std::fs;

mod app;
mod error;
use error::ConfigrsError;

fn run() -> Result<(), ConfigrsError>{
    // setting up arguments and whatnot
    let args = app::build().get_matches();

    // loading either from confys default, or a user supplied config
    let mut cfg: app::Config = match app::load_cfg(args.value_of("config")) {
        Ok(cfg) => cfg,
        Err(_) => return Err(ConfigrsError::BadConfig)
    };

    let mut changed_config: bool = false;
    

    // match which subcommand has been used
    match args.subcommand() {
        ("edit", Some(edit_args)) => {
            let file = edit_args.value_of("file")
                    .expect("file somehow missing, clap.");
            match edit(&mut cfg, file) {
                Ok(()) => Ok(()),
                Err(_) => return Err(ConfigrsError::IDK)
            };
        },
        ("add", Some(add_args)) => {
            let name = add_args.value_of("name")
                .expect("name somehow missing, clap.");
            let file = add_args.value_of("file")
                .expect("file somehow missing, clap.");
            add(
                &mut cfg, 
                name,
                file
            );
            changed_config = true;
        },
        ("remove", Some(remove_args)) => {
            let file = remove_args.value_of("file")
                .expect("name somehow missing, clap.");
            remove(
                &mut cfg, 
                file
            );
            changed_config = true;
        },
        ("yoink", Some(yoink_args)) => {
            let directory = PathBuf::from(yoink_args.value_of("directory")
                .expect("directory somehow missing, clap."));

            match yoink(
                &cfg, 
                directory
            ) { 
                Ok(()) => Ok(()),
                Err(_) => return Err(ConfigrsError::IDK)
            };
        },
        ("yeet", Some(yeet_args)) => {
            let dry_run = yeet_args.is_present("dry_run");
            let directory = PathBuf::from(yeet_args.value_of("directory")
                .expect("directory somehow missing, clap."));

            match yeet(
                &cfg, 
                dry_run,
                directory,
            ) {
                Ok(()) => Ok(()),
                Err(_) => Err(ConfigrsError::IDK)
            };
        },
        _ => unreachable!()
    }



    // if we changed the config (add, remove) then we need to store it again
    if changed_config {
        app::store_cfg(args.value_of("config"), cfg)?;
    }

    // everything went ok
    Ok(())
}

fn yoink(cfg: &app::Config, directory: PathBuf) -> Result<(), std::io::Error> {
    for (name, file) in &cfg.repo {
        let mut dir = directory.clone();
        dir.push(&name);
        println!("Moving {:?} to {:?}", &file, &dir);
        fs::copy(file, dir)?;
    }

    Ok(())
}

fn yeet(
    cfg: &app::Config, 
    dry_run: bool, 
    directory: PathBuf
) -> Result<(), std::io::Error> {

    if dry_run { 
        println!("Doing a dry run");
    }
    for (name, file) in &cfg.repo {
        let mut dir = directory.clone();
        dir.push(&name);
        println!("Moving file {:?} to {:?}", &dir, &file);
        if !dry_run {
            fs::copy(dir, file)?;
        }
    }

    Ok(())
}

fn remove(cfg: &mut app::Config, name: &str) {
    let res = cfg.repo.remove(&name.to_string());

    if let Some(path) = res {
        println!("Removed {} => {:?}", name, path);
    } else {
        println!("{} is not in the repository", name);
    }
}

fn add(cfg: &mut app::Config, name: &str, file: &str) {
    cfg.repo.insert(name.to_string(), PathBuf::from(file));
    println!("Added {} => {} to the repository", name, file);
}

fn edit(cfg: &mut app::Config, name: &str) -> Result<(), String> {
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
