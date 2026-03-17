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
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::Node => "node",
            Self::Go => "go",
            Self::Lua => "lua",
            Self::Cpp => "cpp",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Stack {
    Basic,
    Express,
    Fastapi,
    Hardhat,
    Huggingface,
    React,
    Vite,
    LlamaCpp,
}

impl Stack {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Basic => "basic",
            Self::Express => "express",
            Self::Fastapi => "fastapi",
            Self::Hardhat => "hardhat",
            Self::Huggingface => "huggingface",
            Self::React => "react",
            Self::Vite => "vite",
            Self::LlamaCpp => "llama-cpp",
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
