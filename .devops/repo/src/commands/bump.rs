use crate::utils::print_applications;
use std::process::Command;

pub(crate) fn run(apps: Vec<String>, root: String) {
    let mut success: Vec<String> = vec![];

    let toolchain = "fuel-toolchain.toml".to_string();
    let tmp_toolchain = "fuel-toolchain2.toml".to_string();

    for app in apps {
        println!("\nBumping {}", app);

        // TODO: safety
        let project = std::fs::canonicalize(format!("{}/{}/project", root, app)).unwrap();

        let result = Command::new("mv")
            .arg(format!("{}/{}", project.clone(), toolchain.clone()))
            .arg(format!("{}/{}", project.clone(), tmp_toolchain.clone()))
            .arg("cp")
            .arg(format!("./{}", toolchain.clone()))
            .arg(format!("{}/{}", project.clone(), toolchain.clone()))
            .current_dir(project.clone())
            .arg("forc")
            .arg("build")
            .status();

        match result {
            Ok(status) => {
                if status.success() {
                    success.push(app.clone());
                    let _ = Command::new("rm").arg(tmp_toolchain).status();
                } else {
                    let _ = Command::new("mv")
                        .arg(format!("{}/{}", project.clone(), tmp_toolchain))
                        .arg(format!("{}/{}", project, toolchain));
                }
            }
            Err(_) => {
                let _ = Command::new("mv")
                    .arg(format!("{}/{}", project.clone(), tmp_toolchain))
                    .arg(format!("{}/{}", project, toolchain));
            }
        }
    }

    print_applications(success, "Bumped".to_string());
}
