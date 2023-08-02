use crate::utils::{execute, print_applications, project_path};
use std::process::Command;

const MESSAGE: &str = "Errors found in";

pub(crate) fn run(apps: Vec<String>, root: String) -> anyhow::Result<()> {
    let mut errors: Vec<String> = vec![];

    for app in apps {
        println!("\nTesting {}", app);

        let project = project_path(app.clone(), root.clone())?;

        execute(
            Command::new("cargo")
                .current_dir(project)
                .args(["test", "--color", "always", "-q"]),
            &mut errors,
            &app,
        );
    }

    print_applications(errors, MESSAGE.into());
    Ok(())
}
