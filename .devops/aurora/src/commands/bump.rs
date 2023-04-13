use crate::utils::{print_applications, project_path};
use std::process::Command;

const MESSAGE: &str = "Bumped";

pub(crate) fn run(apps: Vec<String>, root: String) -> anyhow::Result<()> {
    let mut success: Vec<String> = vec![];

    let toolchain = "fuel-toolchain.toml".to_string();
    let tmp_toolchain = "fuel-toolchain2.toml".to_string();

    for app in apps {
        println!("\nBumping {}", app);

        let project = project_path(app.clone(), root.clone())?;

        let _ = Command::new("mv")
            .env("IFS", "''")
            .args([
                format!("{}/{}", project.clone().display(), toolchain.clone()),
                format!("{}/{}", project.clone().display(), tmp_toolchain.clone()),
            ])
            .status();

        let _ = Command::new("cp")
            .env("IFS", "''")
            .args([
                format!("./{}", toolchain.clone()),
                format!("{}/{}", project.clone().display(), toolchain.clone()),
            ])
            .status();

        match Command::new("forc")
            .current_dir(project.clone())
            .arg("build")
            .status()
        {
            Ok(status) => {
                if status.success() {
                    success.push(app.clone());
                    let _ = Command::new("rm")
                        .current_dir(project.clone())
                        .arg(tmp_toolchain.clone())
                        .status();
                } else {
                    let _ = Command::new("mv")
                        .env("IFS", "''")
                        .current_dir(project.clone())
                        .args([
                            format!("{}/{}", project.clone().display(), tmp_toolchain.clone()),
                            format!("{}/{}", project.display(), toolchain.clone()),
                        ])
                        .status();
                }
            }
            Err(_) => {
                let _ = Command::new("mv")
                    .env("IFS", "''")
                    .current_dir(project.clone())
                    .args([
                        format!("{}/{}", project.clone().display(), tmp_toolchain.clone()),
                        format!("{}/{}", project.display(), toolchain.clone()),
                    ])
                    .status();
            }
        }
    }

    print_applications(success, MESSAGE.into());
    Ok(())
}
