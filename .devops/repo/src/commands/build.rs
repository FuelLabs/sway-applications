use crate::utils::{read_applications, repo_root};
use std::process::Command;

pub(crate) fn run() {
    let root = repo_root();
    let apps = read_applications();

    let mut errors: Vec<String> = vec![];

    for app in apps {
        println!("\nBuilding {}", app);

        // TODO: safety
        let project = std::fs::canonicalize(format!("{}/{}/project", root, app)).unwrap();

        let result = Command::new("forc")
            .current_dir(project)
            .arg("build")
            .status();
        match result {
            Ok(status) => {
                if !status.success() {
                    errors.push(app.clone());
                }
            }
            Err(_) => errors.push(app.clone()),
        }
    }

    if 0 < errors.len() {
        println!("\nErrors found in");
        for app in errors.iter() {
            println!("    {}", app);
        }
    }
}
