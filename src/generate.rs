use std::fs;
use std::path::{Path, PathBuf};

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Select};
use directories::ProjectDirs;
use serde_json::Value;

use crate::utils::{is_special_file, SpecialFile};

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
    copy_dir_all(&selected_template, &dst, &project_name)?;

    Ok(())
}

fn copy_dir_all<P: AsRef<Path>>(src: &P, dst: &P, project_name: &str) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_name = entry.file_name();
        if IGNORE_FILES.contains(&file_name.to_str().unwrap()) {
            continue;
        }
        let t = entry.file_type()?;
        if t.is_dir() {
            copy_dir_all(&entry.path(), &dst.as_ref().join(file_name), project_name)?;
        } else {
            match is_special_file(&file_name) {
                Some(SpecialFile::PackageJSON) => {
                    let mut package_json: Value =
                        serde_json::from_str(&fs::read_to_string(entry.path())?)
                            .expect("Cannot parse package.json");
                    package_json["name"] = Value::String(project_name.to_string());
                    let content = serde_json::to_string_pretty(&package_json)
                        .expect("Failed to serialize package.json");
                    fs::write(dst.as_ref().join(&file_name), content)?;
                }
                _ => {
                    fs::copy(entry.path(), dst.as_ref().join(file_name))?;
                }
            }
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
