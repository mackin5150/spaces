use crate::commands::bootstrap;
use crate::error::SpacesError;
use crate::models::workspace_request::WorkspaceRequest;
use crate::workspace::manager::get_adapter;
use anyhow::{Result, anyhow};
use std::fs;

pub fn run(request: WorkspaceRequest) -> Result<()> {
    let adapter = get_adapter(&request.language, &request.stack).ok_or_else(|| {
        anyhow!(SpacesError::UnsupportedWorkspace(format!(
            "{}",
            request.summary()
        )))
    })?;

    let root = request.path.clone();

    if root.exists() {
        return Err(anyhow!(SpacesError::DirectoryAlreadyExists(
            root.display().to_string()
        )));
    }

    fs::create_dir_all(&root)?;
    adapter.create(&root, &request)?;
    let bootstrap_steps = bootstrap::run(&request)?;

    println!(
        "Created {} {} workspace at {}",
        request.language.as_str(),
        request.stack.as_str(),
        root.display()
    );
    println!("Next:");
    println!("  cd {}", root.display());
    println!("  spaces doctor");
    if !bootstrap_steps.is_empty() {
        println!();
        println!("Bootstrap:");
        for step in bootstrap_steps {
            println!("  {}", step);
        }
    }

    Ok(())
}
