use crate::core::fs::ensure_dir;
use crate::models::workspace_request::{Language, PackageManager, Stack, WorkspaceRequest};
use crate::workspace::scaffold::{render_stack_templates, write_manifest};
use crate::workspace::traits::WorkspaceAdapter;
use anyhow::Result;
use std::path::Path;

pub struct NodeHardhatWorkspace;

impl NodeHardhatWorkspace {
    pub fn new() -> Self {
        Self
    }
}

impl WorkspaceAdapter for NodeHardhatWorkspace {
    fn language(&self) -> Language {
        Language::Node
    }

    fn stack(&self) -> Stack {
        Stack::Hardhat
    }

    fn package_manager(&self) -> PackageManager {
        PackageManager::Npm
    }

    fn description(&self) -> &'static str {
        "Hardhat starter with contracts/, scripts/, tests/, dotenv, and optional Dockerfile"
    }

    fn create(&self, root: &Path, request: &WorkspaceRequest) -> Result<()> {
        ensure_dir(&root.join(".spaces"))?;
        ensure_dir(&root.join("contracts"))?;
        ensure_dir(&root.join("scripts"))?;
        ensure_dir(&root.join("ignition").join("modules"))?;
        render_stack_templates(root, request, "node", "hardhat")?;
        write_manifest(root, request)?;
        Ok(())
    }

    fn doctor(&self, root: &Path) -> Result<Vec<String>> {
        Ok(vec![
            check(root, "package.json", "package.json"),
            check(root, "hardhat.config.ts", "hardhat.config.ts"),
            check(root, "contracts", "contracts/ directory"),
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
