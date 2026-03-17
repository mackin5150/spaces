use crate::core::fs::ensure_dir;
use crate::models::workspace_request::{Language, PackageManager, Stack, WorkspaceRequest};
use crate::workspace::scaffold::{render_stack_templates, write_manifest};
use crate::workspace::traits::WorkspaceAdapter;
use anyhow::Result;
use std::path::Path;

pub struct NodeReactWorkspace;

impl NodeReactWorkspace {
    pub fn new() -> Self {
        Self
    }
}

impl WorkspaceAdapter for NodeReactWorkspace {
    fn language(&self) -> Language {
        Language::Node
    }

    fn stack(&self) -> Stack {
        Stack::React
    }

    fn package_manager(&self) -> PackageManager {
        PackageManager::Npm
    }

    fn description(&self) -> &'static str {
        "React workspace with src/, public/, package.json, and env example"
    }

    fn create(&self, root: &Path, request: &WorkspaceRequest) -> Result<()> {
        ensure_dir(&root.join(".spaces"))?;
        ensure_dir(&root.join("src"))?;
        ensure_dir(&root.join("public"))?;
        render_stack_templates(root, request, "node", "react")?;
        write_manifest(root, request)?;
        Ok(())
    }

    fn doctor(&self, root: &Path) -> Result<Vec<String>> {
        let mut checks = Vec::new();

        if root.join("package.json").exists() {
            checks.push("OK  package.json found".to_string());
        } else {
            checks.push("ERR missing package.json".to_string());
        }

        if root.join("src").exists() {
            checks.push("OK  src/ directory found".to_string());
        } else {
            checks.push("ERR missing src/ directory".to_string());
        }

        if root.join("public").exists() {
            checks.push("OK  public/ directory found".to_string());
        } else {
            checks.push("ERR missing public/ directory".to_string());
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
