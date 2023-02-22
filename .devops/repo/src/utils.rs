use std::env;
use std::{fs, process::Command};

pub(crate) fn read_applications() -> Vec<String> {
    let contents = fs::read_to_string("./apps.txt").expect("Unable to read apps.txt file");
    let apps: Vec<&str> = contents.lines().collect();
    let apps = apps.iter().map(|&app| app.into()).collect();
    apps
}

pub(crate) fn repo_root() -> String {
    let project = env::current_dir().unwrap();
    let absolute_path = project.to_str().expect("Failed to convert root path");
    // TODO: safety
    let root = absolute_path.split("/.devops").next().unwrap();
    root.into()
}
