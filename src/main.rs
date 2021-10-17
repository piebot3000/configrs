use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

mod app;
fn run() -> Result<()> {
    // setting up arguments and whatnot
    let args = app::build().get_matches();

    // loading either from confys default, or a user supplied config
    let mut cfg: app::Config = app::load_cfg(args.value_of("config"))?;

    // check to see if we need to rewrite the config before exiting
    let mut changed_config: bool = false;

    // match which subcommand has been used
    match args.subcommand() {
        ("edit", Some(edit_args)) => {
            let file = edit_args
                .value_of("file")
                .expect("file somehow missing, clap.");

            edit(&mut cfg, file)?;
        }

        ("add", Some(add_args)) => {
            let name = add_args
                .value_of("name")
                .expect("name somehow missing, clap.");
            let file = add_args
                .value_of("file")
                .expect("file somehow missing, clap.");

            add(&mut cfg, name, file);
            changed_config = true;
        }

        ("remove", Some(remove_args)) => {
            let file = remove_args
                .value_of("file")
                .expect("name somehow missing, clap.");

            remove(&mut cfg, file)?;
            changed_config = true;
        }

        ("yoink", Some(yoink_args)) => {
            let directory = PathBuf::from(
                yoink_args
                    .value_of("directory")
                    .expect("directory somehow missing, clap."),
            );

            yoink(&cfg, directory)?;
        }

        ("yeet", Some(yeet_args)) => {
            let dry_run = yeet_args.is_present("dry_run");
            let directory = PathBuf::from(
                yeet_args
                    .value_of("directory")
                    .expect("directory somehow missing, clap."),
            );

            yeet(&cfg, dry_run, directory)?;
        }
        _ => unreachable!(),
    }

    // if we changed the config (add, remove) then we need to store it again
    if changed_config {
        app::store_cfg(args.value_of("config"), cfg)?;
    }

    // everything went ok
    Ok(())
}

fn yoink(cfg: &app::Config, directory: PathBuf) -> Result<()> {
    for (name, file) in &cfg.repo {
        let mut dir = directory.clone();
        dir.push(&name);
        println!("Moving {:?} to {:?}", &file, &dir);
        fs::copy(file, dir)?;
    }

    Ok(())
}

fn yeet(cfg: &app::Config, dry_run: bool, directory: PathBuf) -> Result<()> {
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

fn remove(cfg: &mut app::Config, name: &str) -> Result<()> {
    let res = cfg.repo.remove(&name.to_string());

    if let Some(path) = res {
        println!("Removed {} => {:?}", name, path);
        Ok(())
    } else {
        bail!("{} is not in the repository", name);
    }
}

fn add(cfg: &mut app::Config, name: &str, file: &str) {
    cfg.repo.insert(name.to_string(), PathBuf::from(file));
    println!("Added {} => {} to the repository", name, file);
}

fn edit(cfg: &mut app::Config, name: &str) -> Result<()> {
    let path = cfg
        .repo
        .get(name)
        .ok_or_else(|| anyhow!("{} is not in the repository", name))?;

    edit::edit_file(path)?;
    Ok(())
}

fn main() -> Result<()> {
    run()
}
