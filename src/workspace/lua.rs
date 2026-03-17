use crate::core::fs::ensure_dir;
use crate::models::workspace_request::{Language, PackageManager, Stack, WorkspaceRequest};
use crate::workspace::scaffold::{render_stack_templates, write_manifest};
use crate::workspace::traits::WorkspaceAdapter;
use anyhow::Result;
use std::path::Path;

pub struct LuaWorkspace;

impl LuaWorkspace {
    pub fn new() -> Self {
        Self
    }
}

impl WorkspaceAdapter for LuaWorkspace {
    fn language(&self) -> Language {
        Language::Lua
    }

    fn stack(&self) -> Stack {
        Stack::Basic
    }

    fn package_manager(&self) -> PackageManager {
        PackageManager::Make
    }

    fn description(&self) -> &'static str {
        "Minimal Lua project with src/, tests/, Makefile, and optional Dockerfile"
    }

    fn create(&self, root: &Path, request: &WorkspaceRequest) -> Result<()> {
        ensure_dir(&root.join(".spaces"))?;
        ensure_dir(&root.join("src"))?;
        render_stack_templates(root, request, "lua", "basic")?;
        write_manifest(root, request)?;
        Ok(())
    }

    fn doctor(&self, root: &Path) -> Result<Vec<String>> {
        Ok(vec![
            check(root, "src", "src/ directory"),
            check(root, "Makefile", "Makefile"),
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
