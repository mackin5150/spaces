use crate::core::fs::ensure_dir;
use crate::models::workspace_request::{Language, PackageManager, Stack, WorkspaceRequest};
use crate::workspace::scaffold::{render_stack_templates, write_manifest};
use crate::workspace::traits::WorkspaceAdapter;
use anyhow::Result;
use std::path::Path;

pub struct CppLlamaCppWorkspace;

impl CppLlamaCppWorkspace {
    pub fn new() -> Self {
        Self
    }
}

impl WorkspaceAdapter for CppLlamaCppWorkspace {
    fn language(&self) -> Language {
        Language::Cpp
    }

    fn stack(&self) -> Stack {
        Stack::LlamaCpp
    }

    fn package_manager(&self) -> PackageManager {
        PackageManager::Make
    }

    fn description(&self) -> &'static str {
        "llama.cpp runner workspace with prompts/, scripts/, env config, and optional CUDA notes"
    }

    fn create(&self, root: &Path, request: &WorkspaceRequest) -> Result<()> {
        ensure_dir(&root.join(".spaces"))?;
        ensure_dir(&root.join("models"))?;
        ensure_dir(&root.join("prompts"))?;
        ensure_dir(&root.join("scripts"))?;
        ensure_dir(&root.join("vendor"))?;
        render_stack_templates(root, request, "cpp", "llama-cpp")?;
        write_manifest(root, request)?;
        Ok(())
    }

    fn doctor(&self, root: &Path) -> Result<Vec<String>> {
        Ok(vec![
            check(root, "Makefile", "Makefile"),
            check(root, "prompts", "prompts/ directory"),
            check(root, "scripts/run-local.sh", "scripts/run-local.sh"),
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
