use crate::core::fs::ensure_dir;
use crate::models::workspace_request::{Language, PackageManager, Stack, WorkspaceRequest};
use crate::workspace::scaffold::{render_stack_templates, write_manifest};
use crate::workspace::traits::WorkspaceAdapter;
use anyhow::Result;
use std::path::Path;

pub struct PythonFastapiWorkspace;

impl PythonFastapiWorkspace {
    pub fn new() -> Self {
        Self
    }
}

impl WorkspaceAdapter for PythonFastapiWorkspace {
    fn language(&self) -> Language {
        Language::Python
    }

    fn stack(&self) -> Stack {
        Stack::Fastapi
    }

    fn package_manager(&self) -> PackageManager {
        PackageManager::Uv
    }

    fn description(&self) -> &'static str {
        "FastAPI backend starter with app/, health route, uvicorn config, and optional Dockerfile"
    }

    fn create(&self, root: &Path, request: &WorkspaceRequest) -> Result<()> {
        ensure_dir(&root.join(".spaces"))?;
        ensure_dir(&root.join("app"))?;
        render_stack_templates(root, request, "python", "fastapi")?;
        write_manifest(root, request)?;
        Ok(())
    }

    fn doctor(&self, root: &Path) -> Result<Vec<String>> {
        Ok(vec![
            check(root, "pyproject.toml", "pyproject.toml"),
            check(root, "app", "app/ directory"),
            check(root, "app/main.py", "app/main.py"),
            check(root, ".env.example", ".env.example"),
            check(root, ".spaces/workspace.yaml", "workspace manifest"),
        ])
    }
}

fn check(root: &Path, path: &str, label: &str) -> String {
    if root.join(path).exists() {
        format!("OK  {} found", label)
    } else {
        format!("ERR missing {}", label)
    }
}
