use crate::utils::{execute, print_applications};
use anyhow::anyhow;
use std::process::Command;

const SWAY_MESSAGE: &str = "Sway formatting errors in";
const CARGO_MESSAGE: &str = "Cargo formatting errors in";

pub(crate) fn run(apps: Vec<String>, root: String) -> anyhow::Result<()> {
    let mut sway_errors: Vec<String> = vec![];
    let mut cargo_errors: Vec<String> = vec![];

    for app in apps {
        println!("\nFormatting {}", app);

        let project =
            std::fs::canonicalize(format!("{}/{}/project", root, app)).map_err(|error| {
                anyhow!(
                    "Failed to canonicalize path to project for app '{}': {}",
                    app,
                    error
                )
            })?;

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

    print_applications(sway_errors, SWAY_MESSAGE.into());
    print_applications(cargo_errors, CARGO_MESSAGE.into());
    Ok(())
}
