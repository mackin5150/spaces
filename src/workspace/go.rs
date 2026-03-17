use crate::core::fs::ensure_dir;
use crate::models::workspace_request::{Language, PackageManager, Stack, WorkspaceRequest};
use crate::workspace::scaffold::{render_stack_templates, write_manifest};
use crate::workspace::traits::WorkspaceAdapter;
use anyhow::Result;
use std::path::Path;

pub struct GoWorkspace;

impl GoWorkspace {
    pub fn new() -> Self {
        Self
    }
}

impl WorkspaceAdapter for GoWorkspace {
    fn language(&self) -> Language {
        Language::Go
    }

    fn stack(&self) -> Stack {
        Stack::Basic
    }

    fn package_manager(&self) -> PackageManager {
        PackageManager::Go
    }

    fn description(&self) -> &'static str {
        "Minimal Go module with cmd/, internal/, tests, and optional Dockerfile"
    }

    fn create(&self, root: &Path, request: &WorkspaceRequest) -> Result<()> {
        ensure_dir(&root.join(".spaces"))?;
        ensure_dir(&root.join("cmd").join("app"))?;
        ensure_dir(&root.join("internal").join("app"))?;
        render_stack_templates(root, request, "go", "basic")?;
        write_manifest(root, request)?;
        Ok(())
    }

    fn doctor(&self, root: &Path) -> Result<Vec<String>> {
        let mut checks = Vec::new();
        checks.push(check_file(root, "go.mod", "go.mod"));
        checks.push(check_dir(root, "cmd", "cmd/ directory"));
        checks.push(check_dir(root, "internal", "internal/ directory"));
        checks.push(check_file(
            root,
            ".spaces/workspace.yaml",
            "workspace manifest",
        ));
        Ok(checks)
    }
}

fn check_file(root: &Path, path: &str, label: &str) -> String {
    if root.join(path).exists() {
        format!("OK  {} found", label)
    } else {
        format!("ERR missing {}", label)
    }
}

fn check_dir(root: &Path, path: &str, label: &str) -> String {
    if root.join(path).exists() {
        format!("OK  {} found", label)
    } else {
        format!("ERR missing {}", label)
    }
}
