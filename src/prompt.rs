use std::path::PathBuf;

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub dst: PathBuf,
    pub template_path: PathBuf,
}

impl Project {
    pub fn new(name: String, dst: PathBuf, template_path: PathBuf) -> Project {
        Project {
            name,
            dst,
            template_path,
        }
    }
}

pub struct Prompt {
    theme: ColorfulTheme,
    templates: Vec<String>,
    templates_path: PathBuf,
    pub is_monorepo: bool,
    pub projects: Vec<Project>,
    pub monorepo_name: String,
}

impl Prompt {
    pub fn new(templates_path: PathBuf, templates: Vec<String>) -> Prompt {
        Prompt {
            theme: ColorfulTheme::default(),
            is_monorepo: false,
            templates,
            templates_path,
            projects: vec![],
            monorepo_name: String::new(),
        }
    }

    pub fn confirm_is_monorepo(mut self) -> Prompt {
        self.is_monorepo = Confirm::with_theme(&self.theme)
            .with_prompt("Is your project using monorepo?")
            .default(false)
            .interact()
            .unwrap();
        self
    }

    pub fn enter_monorepo_name(mut self) -> Prompt {
        if !self.is_monorepo {
            return self;
        }

        let monorepo_name: String = Input::with_theme(&self.theme)
            .with_prompt("What's your monorepo's name")
            .interact_text()
            .expect("Failed to read the monorepo's name");

        self.monorepo_name = monorepo_name;
        self
    }

    pub fn setup_projects(mut self) -> Prompt {
        if !self.is_monorepo {
            let template_path =
                select_template_path(&self.theme, &self.templates_path, &self.templates);
            let name = enter_project_name(&self.theme, false);
            let dst = PathBuf::from(&name);
            self.projects.push(Project::new(name, dst, template_path));

            return self;
        }

        loop {
            let project_name = enter_project_name(&self.theme, true);
            if project_name.to_lowercase() == "exit" {
                break;
            }
            let template_path =
                select_template_path(&self.theme, &self.templates_path, &self.templates);
            let dst = PathBuf::from(&self.monorepo_name).join(&project_name);

            self.projects
                .push(Project::new(project_name, dst, template_path));
        }

        self
    }
}

fn select_template_path(
    theme: &ColorfulTheme,
    templates_path: &PathBuf,
    templates: &Vec<String>,
) -> PathBuf {
    let idx = Select::with_theme(theme)
        .with_prompt("Which template do you want to use?")
        .default(0)
        .items(templates)
        .interact()
        .expect("Failed to select a template.");

    templates_path.join(templates[idx].to_string())
}

fn enter_project_name(theme: &ColorfulTheme, with_default: bool) -> String {
    let mut input = Input::with_theme(theme).with_prompt("What's your project's name?");
    if with_default {
        input = input.default("exit".to_string());
    }
    input
        .interact_text()
        .expect("Failed to get the project name.")
}
