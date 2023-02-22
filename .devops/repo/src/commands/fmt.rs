use crate::utils::{execute, print_application_errors, read_applications, repo_root};
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

    print_application_errors(sway_errors, "Sway formatting errors in".to_string());
    print_application_errors(cargo_errors, "Cargo formatting errors in".to_string());
}
