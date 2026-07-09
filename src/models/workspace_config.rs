use crate::models::workspace_request::{Language, PackageManager, Stack};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub name: String,
    #[serde(default)]
    pub language: Option<Language>,
    #[serde(default)]
    pub stack: Option<Stack>,
    pub version: u32,
    pub package_manager: String,
    pub created_with: String,
    #[serde(default)]
    pub features: Vec<String>,
    #[serde(default)]
    pub kind: Option<String>,
}

impl WorkspaceConfig {
    pub fn language(&self) -> Option<Language> {
        self.language.clone().or_else(|| {
            self.kind.as_deref().and_then(|kind| match kind {
                "python" | "fastapi" | "flask" | "django" | "huggingface"
                | "streamlit" | "gradio" | "langchain" | "pytorch" => Some(Language::Python),
                "react" | "hardhat" | "express" | "vite" | "nextjs" | "sveltekit"
                | "nestjs" | "astro" | "remix" | "nuxt" | "expo" => {
                    Some(Language::Node)
                }
                "llama-cpp" => Some(Language::Cpp),
                "axum" => Some(Language::Rust),
                "foundry" => Some(Language::Solidity),
                "anchor" => Some(Language::Rust),
                "springboot" => Some(Language::Java),
                _ => None,
            })
        })
    }

    pub fn stack(&self) -> Option<Stack> {
        self.stack.clone().or_else(|| {
            self.kind.as_deref().and_then(|kind| match kind {
                "python" => Some(Stack::Basic),
                "django" => Some(Stack::Django),
                "fastapi" => Some(Stack::Fastapi),
                "flask" => Some(Stack::Flask),
                "huggingface" => Some(Stack::Huggingface),
                "nextjs" => Some(Stack::Nextjs),
                "react" => Some(Stack::React),
                "sveltekit" => Some(Stack::Sveltekit),
                "hardhat" => Some(Stack::Hardhat),
                "express" => Some(Stack::Express),
                "vite" => Some(Stack::Vite),
                "llama-cpp" => Some(Stack::LlamaCpp),
                // AI/ML
                "streamlit" => Some(Stack::Streamlit),
                "gradio" => Some(Stack::Gradio),
                "langchain" => Some(Stack::Langchain),
                "pytorch" => Some(Stack::Pytorch),
                // Node/Frontend
                "nestjs" => Some(Stack::Nestjs),
                "astro" => Some(Stack::Astro),
                "remix" => Some(Stack::Remix),
                "nuxt" => Some(Stack::Nuxt),
                "expo" => Some(Stack::Expo),
                // Smart contracts
                "foundry" => Some(Stack::Foundry),
                "anchor" => Some(Stack::Anchor),
                // Go
                "gin" => Some(Stack::Gin),
                "grpc-go" => Some(Stack::GrpcGo),
                // Other
                "axum" => Some(Stack::Axum),
                "springboot" => Some(Stack::Springboot),
                _ => None,
            })
        })
    }

    pub fn package_manager_enum(&self) -> Option<PackageManager> {
        match self.package_manager.as_str() {
            "npm" => Some(PackageManager::Npm),
            "pnpm" => Some(PackageManager::Pnpm),
            "yarn" => Some(PackageManager::Yarn),
            "uv" => Some(PackageManager::Uv),
            "pip" => Some(PackageManager::Pip),
            "go" => Some(PackageManager::Go),
            "make" => Some(PackageManager::Make),
            "cargo" => Some(PackageManager::Cargo),
            "forge" => Some(PackageManager::Forge),
            _ => None,
        }
    }
}
