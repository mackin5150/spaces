use crate::core::fs::ensure_dir;
use crate::models::workspace_request::{Language, PackageManager, Stack, WorkspaceRequest};
use crate::workspace::scaffold::{render_stack_templates, write_manifest};
use crate::workspace::traits::WorkspaceAdapter;
use anyhow::Result;
use std::path::Path;

pub struct HuggingFaceWorkspace;

impl HuggingFaceWorkspace {
    pub fn new() -> Self {
        Self
    }
}

impl WorkspaceAdapter for HuggingFaceWorkspace {
    fn language(&self) -> Language {
        Language::Python
    }

    fn stack(&self) -> Stack {
        Stack::Huggingface
    }

    fn package_manager(&self) -> PackageManager {
        PackageManager::Uv
    }

    fn description(&self) -> &'static str {
        "Hugging Face workspace with models/, data/, notebooks/, and env tuning"
    }

    fn create(&self, root: &Path, request: &WorkspaceRequest) -> Result<()> {
        ensure_dir(&root.join(".spaces"))?;
        ensure_dir(&root.join("models"))?;
        ensure_dir(&root.join("data"))?;
        ensure_dir(&root.join("notebooks"))?;
        render_stack_templates(root, request, "python", "huggingface")?;
        write_manifest(root, request)?;
        Ok(())
    }

    fn doctor(&self, root: &Path) -> Result<Vec<String>> {
        let mut checks = Vec::new();

        if root.join("requirements.txt").exists() {
            checks.push("OK  requirements.txt found".to_string());
        } else {
            checks.push("ERR missing requirements.txt".to_string());
        }

        if root.join("models").exists() {
            checks.push("OK  models/ directory found".to_string());
        } else {
            checks.push("ERR missing models/ directory".to_string());
        }

        if root.join("data").exists() {
            checks.push("OK  data/ directory found".to_string());
        } else {
            checks.push("ERR missing data/ directory".to_string());
        }

        if root.join("notebooks").exists() {
            checks.push("OK  notebooks/ directory found".to_string());
        } else {
            checks.push("ERR missing notebooks/ directory".to_string());
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
