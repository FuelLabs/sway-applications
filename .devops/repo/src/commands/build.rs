use crate::utils::{execute, print_applications};
use std::process::Command;

pub(crate) fn run(apps: Vec<String>, root: String) {
    let mut errors: Vec<String> = vec![];

    for app in apps {
        println!("\nBuilding {}", app);

        // TODO: safety
        let project = std::fs::canonicalize(format!("{}/{}/project", root, app)).unwrap();

        execute(
            Command::new("forc").current_dir(project).arg("build"),
            &mut errors,
            &app,
        );
    }

    print_applications(errors, "Errors found in".to_string());
}
