use crate::core::fs::ensure_dir;
use crate::models::workspace_request::{Language, PackageManager, Stack, WorkspaceRequest};
use crate::workspace::scaffold::{render_stack_templates, write_manifest};
use crate::workspace::traits::WorkspaceAdapter;
use anyhow::Result;
use std::path::Path;

pub struct NodeNextjsWorkspace;

impl NodeNextjsWorkspace {
    pub fn new() -> Self {
        Self
    }
}

impl WorkspaceAdapter for NodeNextjsWorkspace {
    fn language(&self) -> Language {
        Language::Node
    }

    fn stack(&self) -> Stack {
        Stack::Nextjs
    }

    fn package_manager(&self) -> PackageManager {
        PackageManager::Npm
    }

    fn description(&self) -> &'static str {
        "Next.js app starter with app router, env config, and optional Dockerfile"
    }

    fn create(&self, root: &Path, request: &WorkspaceRequest) -> Result<()> {
        ensure_dir(&root.join(".spaces"))?;
        ensure_dir(&root.join("app"))?;
        render_stack_templates(root, request, "node", "nextjs")?;
        write_manifest(root, request)?;
        Ok(())
    }

    fn doctor(&self, root: &Path) -> Result<Vec<String>> {
        Ok(vec![
            check(root, "package.json", "package.json"),
            check(root, "next.config.mjs", "next.config.mjs"),
            check(root, "app/page.tsx", "app/page.tsx"),
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
