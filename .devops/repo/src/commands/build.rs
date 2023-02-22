use crate::utils::{execute, print_application_errors, read_applications, repo_root};
use std::process::Command;

pub(crate) fn run() {
    let root = repo_root();
    let apps = read_applications();

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

    print_application_errors(errors, "Errors found in".to_string());
}
