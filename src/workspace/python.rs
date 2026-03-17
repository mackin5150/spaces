use crate::core::fs::ensure_dir;
use crate::models::workspace_request::{Language, PackageManager, Stack, WorkspaceRequest};
use crate::workspace::scaffold::{render_stack_templates, write_manifest};
use crate::workspace::traits::WorkspaceAdapter;
use anyhow::Result;
use std::path::Path;

pub struct PythonWorkspace;

impl PythonWorkspace {
    pub fn new() -> Self {
        Self
    }
}

impl WorkspaceAdapter for PythonWorkspace {
    fn language(&self) -> Language {
        Language::Python
    }

    fn stack(&self) -> Stack {
        Stack::Basic
    }

    fn package_manager(&self) -> PackageManager {
        PackageManager::Uv
    }

    fn description(&self) -> &'static str {
        "Python workspace with src/, tests/, pyproject.toml, and .env.example"
    }

    fn create(&self, root: &Path, request: &WorkspaceRequest) -> Result<()> {
        ensure_dir(&root.join(".spaces"))?;
        ensure_dir(&root.join("src"))?;
        render_stack_templates(root, request, "python", "basic")?;
        write_manifest(root, request)?;
        Ok(())
    }

    fn doctor(&self, root: &Path) -> Result<Vec<String>> {
        let mut checks = Vec::new();

        if root.join("pyproject.toml").exists() {
            checks.push("OK  pyproject.toml found".to_string());
        } else {
            checks.push("ERR missing pyproject.toml".to_string());
        }

        if root.join("src").exists() {
            checks.push("OK  src/ directory found".to_string());
        } else {
            checks.push("ERR missing src/ directory".to_string());
        }

        if root.join(".env.example").exists() {
            checks.push("OK  .env.example found".to_string());
        } else {
            checks.push("ERR missing .env.example".to_string());
        }

        if root.join(".spaces/workspace.yaml").exists() {
            checks.push("OK  workspace manifest found".to_string());
        } else {
            checks.push("ERR missing .spaces/workspace.yaml".to_string());
        }

        Ok(checks)
    }
}
