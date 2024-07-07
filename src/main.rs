use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let templates_path = Path::new("templates");
    let templates = fs::read_dir(templates_path)?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_dir() {
                path.file_name()?.to_str().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();
    let theme = ColorfulTheme::default();
    let selected_template = Select::with_theme(&theme)
        .with_prompt("Which template do you want to use?")
        .default(0)
        .items(&templates)
        .interact()
        .map(|idx| templates_path.join(&templates[idx]))?;
    let project_name: String = Input::with_theme(&theme)
        .with_prompt("What's your project's name?")
        .interact_text()?;

    let dst = Path::new(&project_name);
    empty_folder(dst);
    copy_dir_all(selected_template.as_path(), dst)?;

    println!("Done.");
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let t = entry.file_type()?;
        if t.is_dir() {
            copy_dir_all(
                entry.path().as_path(),
                dst.join(entry.file_name()).as_path(),
            )?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn empty_folder(dst: &Path) {
    if let Err(_) = fs::remove_dir_all(dst) {
        println!("Target folder not exist, create one.")
    };
}
