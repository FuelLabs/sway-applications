use crate::{
    cli::Program,
    utils::{execute, print_applications, project_path},
};
use std::process::Command;

const SWAY_MESSAGE: &str = "Sway build errors in";
const CARGO_MESSAGE: &str = "Cargo build errors in";

pub(crate) fn run(apps: Vec<String>, program: Program, root: String) -> anyhow::Result<()> {
    let mut sway_errors: Vec<String> = vec![];
    let mut cargo_errors: Vec<String> = vec![];

    for app in apps {
        println!("\nBuilding {}", app);

        let project = project_path(app.clone(), root.clone())?;

        match program.clone() {
            Program::All => {
                execute(
                    Command::new("forc")
                        .current_dir(project.clone())
                        .arg("build"),
                    &mut sway_errors,
                    &app,
                );
                execute(
                    Command::new("cargo").current_dir(project).arg("build"),
                    &mut cargo_errors,
                    &app,
                );
            }
            Program::Rust => execute(
                Command::new("cargo").current_dir(project).arg("build"),
                &mut cargo_errors,
                &app,
            ),
            Program::Sway => execute(
                Command::new("forc").current_dir(project).arg("build"),
                &mut sway_errors,
                &app,
            ),
        }
    }

    print_applications(sway_errors, SWAY_MESSAGE.into());
    print_applications(cargo_errors, CARGO_MESSAGE.into());
    Ok(())
}
