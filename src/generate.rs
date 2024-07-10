use std::fs;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde_json::{json, Value};

use crate::prompt::Prompt;
use crate::utils::{
    is_special_file, should_ignore, SpecialFile, PACKAGE_JSON, PNPM_WORKSPACE_YAML,
};

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

    let prompt = Prompt::new(templates_path, templates)
        .confirm_is_monorepo()
        .enter_monorepo_name()
        .setup_projects();

    if prompt.is_monorepo {
        empty_folder(&prompt.monorepo_name);
    }
    for project in prompt.projects.iter() {
        empty_folder(&project.name);
        copy_dir_all(&project.template_path, &project.dst, &project.name)?;
    }

    if prompt.is_monorepo {
        let mut package_json = json!({});
        package_json["name"] = json!(&prompt.monorepo_name);
        let monorepo_path = PathBuf::from(&prompt.monorepo_name);
        fs::write(
            monorepo_path.join(PACKAGE_JSON),
            serde_json::to_string_pretty(&package_json).unwrap(),
        )?;
        let mut workspace_content = String::from("packages:\n");
        for project in prompt.projects.iter() {
            workspace_content.push_str("  - ");
            workspace_content.push_str(&project.name);
            workspace_content.push('\n');
        }
        fs::write(monorepo_path.join(PNPM_WORKSPACE_YAML), workspace_content)?;
    }

    Ok(())
}

fn copy_dir_all<P: AsRef<Path>>(src: &P, dst: &P, project_name: &str) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_name = entry.file_name();
        if should_ignore(&file_name) {
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
    if let Err(e) = fs::remove_dir_all(dst) {
        eprintln!("{}", e);
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
