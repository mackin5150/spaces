use anyhow::{Result, anyhow};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Python,
    Node,
    Go,
    Lua,
    Cpp,
    Rust,
    Solidity,
    Java,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::Node => "node",
            Self::Go => "go",
            Self::Lua => "lua",
            Self::Cpp => "cpp",
            Self::Rust => "rust",
            Self::Solidity => "solidity",
            Self::Java => "java",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Stack {
    Basic,
    Django,
    Express,
    Fastapi,
    Flask,
    Hardhat,
    Huggingface,
    Nextjs,
    React,
    Sveltekit,
    Vite,
    LlamaCpp,
    // AI/ML stacks (Python)
    Streamlit,
    Gradio,
    Langchain,
    Pytorch,
    // Node/Frontend stacks
    Nestjs,
    Astro,
    Remix,
    Nuxt,
    Expo,
    // Smart contract stacks
    Foundry,
    Anchor,
    // Go stacks
    Gin,
    GrpcGo,
    // Other language stacks
    Axum,
    Springboot,
}

impl Stack {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Django => "django",
            Self::Express => "express",
            Self::Fastapi => "fastapi",
            Self::Flask => "flask",
            Self::Hardhat => "hardhat",
            Self::Huggingface => "huggingface",
            Self::Nextjs => "nextjs",
            Self::React => "react",
            Self::Sveltekit => "sveltekit",
            Self::Vite => "vite",
            Self::LlamaCpp => "llama-cpp",
            // AI/ML
            Self::Streamlit => "streamlit",
            Self::Gradio => "gradio",
            Self::Langchain => "langchain",
            Self::Pytorch => "pytorch",
            // Node/Frontend
            Self::Nestjs => "nestjs",
            Self::Astro => "astro",
            Self::Remix => "remix",
            Self::Nuxt => "nuxt",
            Self::Expo => "expo",
            // Smart contracts
            Self::Foundry => "foundry",
            Self::Anchor => "anchor",
            // Go
            Self::Gin => "gin",
            Self::GrpcGo => "grpc-go",
            // Other
            Self::Axum => "axum",
            Self::Springboot => "springboot",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
    Uv,
    Pip,
    Go,
    Make,
    Cargo,
    Forge,
}

impl PackageManager {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Npm => "npm",
            Self::Pnpm => "pnpm",
            Self::Yarn => "yarn",
            Self::Uv => "uv",
            Self::Pip => "pip",
            Self::Go => "go",
            Self::Make => "make",
            Self::Cargo => "cargo",
            Self::Forge => "forge",
        }
    }
}

#[derive(Clone, Debug)]
pub struct WorkspaceRequest {
    pub name: String,
    pub path: PathBuf,
    pub language: Language,
    pub stack: Stack,
    pub package_manager: PackageManager,
    pub install: bool,
    pub docker: bool,
    pub git: bool,
    pub tests: bool,
    pub cuda: bool,
}

impl WorkspaceRequest {
    pub fn from_cli(
        path: String,
        name: Option<String>,
        language: Language,
        stack: Stack,
        package_manager: PackageManager,
        install: bool,
        docker: bool,
        git: bool,
        tests: bool,
        cuda: bool,
    ) -> Result<Self> {
        let path_buf = PathBuf::from(&path);
        let resolved_name = match name {
            Some(name) => name,
            None => default_name_from_path(&path_buf)?,
        };

        Ok(Self {
            name: resolved_name,
            path: path_buf,
            language,
            stack,
            package_manager,
            install,
            docker,
            git,
            tests,
            cuda,
        })
    }

    pub fn feature_list(&self) -> Vec<&'static str> {
        let mut features = Vec::new();

        if self.docker {
            features.push("docker");
        }
        if self.git {
            features.push("git");
        }
        if self.tests {
            features.push("tests");
        }
        if self.cuda {
            features.push("cuda");
        }

        features
    }

    pub fn summary(&self) -> String {
        format!("{} + {}", self.language.as_str(), self.stack.as_str())
    }
}

fn default_name_from_path(path: &Path) -> Result<String> {
    let name = path
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty() && *value != "." && *value != "..")
        .ok_or_else(|| {
            anyhow!(
                "could not derive a project name from path: {}",
                path.display()
            )
        })?;

    Ok(name.to_string())
}
