use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

mod app;
use app::Command;

fn run() -> Result<()> {
    // setting up arguments and whatnot
    let args = app::Opt::from_args();

    // loading either from confys default, or a user supplied config
    let mut cfg: app::Config = app::load_cfg(&args.config_file)?;

    // check to see if we need to rewrite the config before exiting
    let mut changed_config: bool = false;

    // match which subcommand has been used
    match args.cmd {
        Command::Add { name, file } => {
            add(&mut cfg, &name, &file);
            changed_config = true;
        },
        Command::Remove { name } => {
            remove(&mut cfg, &name)?;
            changed_config = true;
        },
        Command::Yoink { directory } => {
            yoink(&cfg, &directory)?;
        },
        Command::Edit { name } => {
            edit(&mut cfg, &name)?;
        },
    }

    // if we changed the config (add, remove) then we need to store it again
    if changed_config {
        app::store_cfg(&args.config_file, cfg)?;
    }

    // everything went ok
    Ok(())
}

fn yoink(cfg: &app::Config, directory: &PathBuf) -> Result<()> {
    for (name, file) in &cfg.repo {
        let mut dir = directory.clone();
        dir.push(&name);
        println!("Moving {:?} to {:?}", &file, &dir);
        fs::copy(file, dir)?;
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

fn add(cfg: &mut app::Config, name: &str, file: &PathBuf) {
    cfg.repo.insert(name.to_string(), PathBuf::from(file));
    println!("Added {} => {:?} to the repository", name, file);
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
