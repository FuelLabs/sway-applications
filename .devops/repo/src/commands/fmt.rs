use crate::utils::{execute, print_applications};
use std::process::Command;

pub(crate) fn run(apps: Vec<String>, root: String) {
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

    print_applications(sway_errors, "Sway formatting errors in".to_string());
    print_applications(cargo_errors, "Cargo formatting errors in".to_string());
}
