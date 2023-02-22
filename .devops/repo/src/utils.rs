use std::process::Command;

pub(crate) fn execute(command: &mut Command, errors: &mut Vec<String>, app: &String) {
    let result = command.status();

    match result {
        Ok(status) => {
            if !status.success() {
                errors.push(app.clone());
            }
        }
        Err(_) => errors.push(app.clone()),
    }
}

pub(crate) fn print_application_errors(errors: Vec<String>, error_message: String) {
    if 0 < errors.len() {
        println!("\n{}", error_message);
        for app in errors.iter() {
            println!("    {}", app);
        }
    }
}

pub(crate) fn read_applications() -> Vec<String> {
    let contents = std::fs::read_to_string("./apps.txt").expect("Unable to read apps.txt file");
    let apps: Vec<&str> = contents.lines().collect();
    let apps = apps.iter().map(|&app| app.into()).collect();
    apps
}

pub(crate) fn repo_root() -> String {
    let project = std::env::current_dir().unwrap();
    let absolute_path = project.to_str().expect("Failed to convert root path");
    // TODO: safety
    let root = absolute_path.split("/.devops").next().unwrap();
    root.into()
}
