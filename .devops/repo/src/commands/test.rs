use crate::utils::{execute, print_application_errors};
use std::process::Command;

pub(crate) fn run(apps: Vec<String>, root: String) {
    let mut errors: Vec<String> = vec![];

    for app in apps {
        println!("\nTesting {}", app);

        // TODO: safety
        let project = std::fs::canonicalize(format!("{}/{}/project", root, app)).unwrap();

        execute(
            Command::new("cargo")
                .current_dir(project)
                .arg("test")
                .arg("--color")
                .arg("always")
                .arg("-q"),
            &mut errors,
            &app,
        );
    }

    print_application_errors(errors, "Errors found in".to_string());
}
