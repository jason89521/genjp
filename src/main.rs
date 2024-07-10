use clap::{Parser, Subcommand};
use std::error::Error;
use std::path::PathBuf;
use std::{env, fs};

mod generate;
mod utils;

use generate::{get_config_dir, prompt};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Set up the templates directory
    SetTemplates {
        #[arg()]
        path: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::SetTemplates { path }) => {
            set_templates_path(path)?;
            return Ok(());
        }
        None => {
            prompt()?;
        }
    }

    println!("Done.");
    Ok(())
}

fn set_templates_path(path: &PathBuf) -> std::io::Result<()> {
    use std::io;
    let path = if path.is_absolute() {
        path
    } else {
        &env::current_dir()?.join(path)
    };
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The templates directory doesn't exist.",
        ));
    }
    let config_dir = get_config_dir().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Cannot find config directory.",
    ))?;
    fs::create_dir_all(&config_dir)?;
    let config_file = config_dir.join("templates_path.txt");
    fs::write(config_file, path.to_str().unwrap())?;
    println!("Templates path set to: {}", path.display());
    Ok(())
}
