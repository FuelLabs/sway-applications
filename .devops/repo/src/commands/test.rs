use crate::utils::{execute, print_applications};
use anyhow::anyhow;
use std::process::Command;

const MESSAGE: &str = "Errors found in";

pub(crate) fn run(apps: Vec<String>, root: String) {
    let mut errors: Vec<String> = vec![];

    for app in apps {
        println!("\nTesting {}", app);

        let project = std::fs::canonicalize(format!("{}/{}/project", root, app))
            .map_err(|error| {
                anyhow!(
                    "Failed to canonicalize path to project for app '{}': {}",
                    app,
                    error
                )
            })
            .unwrap();

        execute(
            Command::new("cargo")
                .current_dir(project)
                .args(["test", "--color", "always", "-q"]),
            &mut errors,
            &app,
        );
    }

    print_applications(errors, MESSAGE.into());
}
