use crate::error::SpacesError;
use crate::models::workspace_config::WorkspaceConfig;
use crate::workspace::manager::get_adapter;
use anyhow::{Result, anyhow};
use std::fs;
use std::path::Path;

pub fn run() -> Result<()> {
    let manifest_path = Path::new(".spaces/workspace.yaml");

    if !manifest_path.exists() {
        return Err(anyhow!(SpacesError::MissingManifest(
            ".spaces/workspace.yaml".to_string()
        )));
    }

    let raw = fs::read_to_string(manifest_path)?;
    let config: WorkspaceConfig = serde_yaml::from_str(&raw)?;
    let language = config
        .language()
        .ok_or_else(|| anyhow!("workspace manifest is missing a supported language"))?;
    let stack = config
        .stack()
        .ok_or_else(|| anyhow!("workspace manifest is missing a supported stack"))?;

    let adapter = get_adapter(&language, &stack).ok_or_else(|| {
        anyhow!(SpacesError::UnsupportedWorkspace(format!(
            "{} + {}",
            language.as_str(),
            stack.as_str()
        )))
    })?;

    println!("Workspace: {}", config.name);
    println!("Language: {}", language.as_str());
    println!("Stack: {}", stack.as_str());
    println!("Version: {}", config.version);
    if let Some(pm) = config.package_manager_enum() {
        println!("Package Manager: {}", pm.as_str());
    }
    println!();

    let checks = adapter.doctor(Path::new("."))?;

    for check in checks {
        println!("{}", check);
    }

    Ok(())
}
