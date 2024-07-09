use std::fs;
use std::path::{Path, PathBuf};

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use directories::ProjectDirs;

const IGNORE_FILES: [&str; 2] = ["node_modules", "pnpm-lock.yaml"];

pub fn prompt() -> std::io::Result<()> {
    let templates_path = get_templates_path()?;
    let templates = fs::read_dir(&templates_path)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_dir() {
                path.file_name()?.to_str().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let theme = ColorfulTheme::default();
    let selected_template = Select::with_theme(&theme)
        .with_prompt("Which template do you want to use?")
        .default(0)
        .items(&templates)
        .interact()
        .map(|idx| templates_path.join(&templates[idx]))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let project_name: String = Input::with_theme(&theme)
        .with_prompt("What's your project's name?")
        .interact_text()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let dst = PathBuf::from(&project_name);
    empty_folder(&dst);
    copy_dir_all(&selected_template, &dst)?;

    Ok(())
}

fn copy_dir_all<P: AsRef<Path>>(src: &P, dst: &P) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        if IGNORE_FILES.contains(&entry.file_name().to_str().unwrap()) {
            continue;
        }
        let t = entry.file_type()?;
        if t.is_dir() {
            copy_dir_all(&entry.path(), &dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn empty_folder<P: AsRef<Path>>(dst: &P) {
    if let Err(_) = fs::remove_dir_all(dst) {
        println!("Target folder not exist, create one.")
    };
}

pub fn get_config_dir() -> Option<PathBuf> {
    ProjectDirs::from("", "", "GenJP").map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
}

fn get_templates_path() -> std::io::Result<PathBuf> {
    use std::io;
    let config_dir = get_config_dir().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Cannot find config directory.",
    ))?;
    println!("config_dir: {}", config_dir.to_str().unwrap());
    let config_file = config_dir.join("templates_path.txt");
    let path = fs::read_to_string(config_file).map_err(|e| {
        eprintln!("Failed to read the templates path.");
        e
    })?;

    Ok(PathBuf::from(path))
}
