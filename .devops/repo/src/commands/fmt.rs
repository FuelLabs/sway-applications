use crate::utils::{read_applications, repo_root};
use std::process::Command;

pub(crate) fn run() {
    let root = repo_root();
    let apps = read_applications();

    let mut sway_errors: Vec<String> = vec![];
    let mut cargo_errors: Vec<String> = vec![];

    for app in apps {
        println!("\nFormatting {}", app);

        // TODO: safety
        let project = std::fs::canonicalize(format!("{}/{}/project", root, app)).unwrap();

        execute(
            Command::new("forc").current_dir(project.clone()).arg("fmt"),
            &mut sway_errors,
            &app,
        );
        execute(
            Command::new("cargo").current_dir(project).arg("fmt"),
            &mut cargo_errors,
            &app,
        );
    }

    print(sway_errors, "Sway".to_string());
    print(cargo_errors, "Cargo".to_string());
}

fn execute(command: &mut Command, errors: &mut Vec<String>, app: &String) {
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

fn print(errors: Vec<String>, source: String) {
    if 0 < errors.len() {
        println!("{}", format!("\n{} formatting errors found in", source));
        for app in errors.iter() {
            println!("    {}", app);
        }
    }
}
