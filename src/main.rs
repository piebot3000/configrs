use std::error::Error;

mod app;

fn run() -> Result<(), Box<dyn Error>>{
    // setting up arguments and whatnot
    let args = app::build().get_matches();

    // loading either from confys default, or a user supplied config
    let mut cfg: app::Config = app::load_cfg(args.value_of("config"))?;

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
            app::store_cfg(args.value_of("config"), cfg)?;
        },
        ("remove", Some(remove_args)) => {
            remove(
                &mut cfg, 
                remove_args.value_of("file").expect("name somehow missing, clap."), 
            );
            app::store_cfg(args.value_of("config"), cfg)?;
        },
        _ => unreachable!()
    }


    // everything went ok
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
    cfg.repo.insert(name.to_string(), file.to_string());
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
