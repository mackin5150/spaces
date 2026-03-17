use crate::core::process;
use crate::models::workspace_request::{PackageManager, WorkspaceRequest};
use anyhow::Result;

pub fn run(request: &WorkspaceRequest) -> Result<Vec<String>> {
    let mut completed = Vec::new();

    if request.git {
        process::run("git", &["init"], &request.path)?;
        completed.push("git init".to_string());
    }

    if request.install {
        let (program, args): (&str, &[&str]) = match request.package_manager {
            PackageManager::Npm => ("npm", &["install"]),
            PackageManager::Pnpm => ("pnpm", &["install"]),
            PackageManager::Yarn => ("yarn", &["install"]),
            PackageManager::Uv => ("uv", &["sync"]),
            PackageManager::Pip => ("pip", &["install", "-r", "requirements.txt"]),
            PackageManager::Go => ("go", &["mod", "tidy"]),
            PackageManager::Make => ("make", &["bootstrap"]),
        };

        process::run(program, args, &request.path)?;
        completed.push(format!("{} {}", program, args.join(" ")));
    }

    Ok(completed)
}
