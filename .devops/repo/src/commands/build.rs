use crate::utils::{execute, print_applications};
use anyhow::anyhow;
use std::process::Command;

const MESSAGE: &str = "Errors found in";

pub(crate) fn run(apps: Vec<String>, root: String) -> anyhow::Result<()> {
    let mut errors: Vec<String> = vec![];

    for app in apps {
        println!("\nBuilding {}", app);

        let project =
            std::fs::canonicalize(format!("{}/{}/project", root, app)).map_err(|error| {
                anyhow!(
                    "Failed to canonicalize path to project for app '{}': {}",
                    app,
                    error
                )
            })?;

        execute(
            Command::new("forc").current_dir(project).arg("build"),
            &mut errors,
            &app,
        );
    }

    print_applications(errors, MESSAGE.into());
    Ok(())
}
