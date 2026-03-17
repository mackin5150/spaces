use crate::core::fs::ensure_dir;
use crate::models::workspace_request::{Language, PackageManager, Stack, WorkspaceRequest};
use crate::workspace::scaffold::{render_stack_templates, write_manifest};
use crate::workspace::traits::WorkspaceAdapter;
use anyhow::Result;
use std::path::Path;

pub struct PythonDjangoWorkspace;

impl PythonDjangoWorkspace {
    pub fn new() -> Self {
        Self
    }
}

impl WorkspaceAdapter for PythonDjangoWorkspace {
    fn language(&self) -> Language {
        Language::Python
    }

    fn stack(&self) -> Stack {
        Stack::Django
    }

    fn package_manager(&self) -> PackageManager {
        PackageManager::Uv
    }

    fn description(&self) -> &'static str {
        "Django starter with manage.py, project package, env config, and optional Dockerfile"
    }

    fn create(&self, root: &Path, request: &WorkspaceRequest) -> Result<()> {
        ensure_dir(&root.join(".spaces"))?;
        ensure_dir(&root.join("config"))?;
        render_stack_templates(root, request, "python", "django")?;
        write_manifest(root, request)?;
        Ok(())
    }

    fn doctor(&self, root: &Path) -> Result<Vec<String>> {
        Ok(vec![
            check(root, "pyproject.toml", "pyproject.toml"),
            check(root, "manage.py", "manage.py"),
            check(root, "config/settings.py", "config/settings.py"),
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
