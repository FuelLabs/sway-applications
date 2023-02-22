use crate::utils::{read_applications, repo_root};
use std::{env::set_current_dir, process::Command};

pub(crate) fn run() {
    let root = repo_root();
    let apps = read_applications();

    let mut errors: Vec<String> = vec![];

    for app in apps {
        println!("\nBuilding {}", app);

        set_current_dir(format!("{}/{}/project", root, app));

        let build = Command::new("forc").arg("build").status();
        match build {
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
