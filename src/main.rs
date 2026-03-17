mod cli;
mod commands;
mod core;
mod error;
mod models;
mod workspace;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use models::workspace_request::{Language, PackageManager, Stack, WorkspaceRequest};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            path,
            name,
            lang,
            stack,
            pm,
            docker,
            git,
            tests,
            cuda,
        } => commands::new::run(build_request(
            path, name, lang, stack, pm, docker, git, tests, cuda,
        )?)?,
        Commands::Python {
            path,
            name,
            stack,
            pm,
            docker,
            git,
            tests,
            cuda,
        } => commands::new::run(build_request(
            path,
            name,
            Language::Python,
            stack,
            pm,
            docker,
            git,
            tests,
            cuda,
        )?)?,
        Commands::Node {
            path,
            name,
            stack,
            pm,
            docker,
            git,
            tests,
        } => commands::new::run(build_request(
            path,
            name,
            Language::Node,
            stack,
            pm,
            docker,
            git,
            tests,
            false,
        )?)?,
        Commands::Go {
            path,
            name,
            stack,
            docker,
            git,
            tests,
        } => commands::new::run(build_request(
            path,
            name,
            Language::Go,
            stack,
            Some(PackageManager::Go),
            docker,
            git,
            tests,
            false,
        )?)?,
        Commands::Lua {
            path,
            name,
            stack,
            docker,
            git,
            tests,
        } => commands::new::run(build_request(
            path,
            name,
            Language::Lua,
            stack,
            Some(PackageManager::Make),
            docker,
            git,
            tests,
            false,
        )?)?,
        Commands::Hardhat {
            path,
            name,
            pm,
            docker,
            git,
            tests,
        } => commands::new::run(build_request(
            path,
            name,
            Language::Node,
            Stack::Hardhat,
            pm,
            docker,
            git,
            tests,
            false,
        )?)?,
        Commands::Express {
            path,
            name,
            pm,
            docker,
            git,
            tests,
        } => commands::new::run(build_request(
            path,
            name,
            Language::Node,
            Stack::Express,
            pm,
            docker,
            git,
            tests,
            false,
        )?)?,
        Commands::Fastapi {
            path,
            name,
            pm,
            docker,
            git,
            tests,
        } => commands::new::run(build_request(
            path,
            name,
            Language::Python,
            Stack::Fastapi,
            pm,
            docker,
            git,
            tests,
            false,
        )?)?,
        Commands::Huggingface {
            path,
            name,
            pm,
            docker,
            git,
            tests,
            cuda,
        } => commands::new::run(build_request(
            path,
            name,
            Language::Python,
            Stack::Huggingface,
            pm,
            docker,
            git,
            tests,
            cuda,
        )?)?,
        Commands::Vite {
            path,
            name,
            pm,
            docker,
            git,
        } => commands::new::run(build_request(
            path,
            name,
            Language::Node,
            Stack::Vite,
            pm,
            docker,
            git,
            false,
            false,
        )?)?,
        Commands::LlamaCpp {
            path,
            name,
            docker,
            git,
            cuda,
        } => commands::new::run(build_request(
            path,
            name,
            Language::Cpp,
            Stack::LlamaCpp,
            Some(PackageManager::Make),
            docker,
            git,
            false,
            cuda,
        )?)?,
        Commands::List => commands::list::run()?,
        Commands::Doctor => commands::doctor::run()?,
    }

    Ok(())
}

fn build_request(
    path: String,
    name: Option<String>,
    language: Language,
    stack: Stack,
    pm: Option<PackageManager>,
    docker: bool,
    git: bool,
    tests: bool,
    cuda: bool,
) -> Result<WorkspaceRequest> {
    let package_manager =
        pm.unwrap_or_else(|| workspace::manager::default_package_manager(&language, &stack));

    WorkspaceRequest::from_cli(
        path,
        name,
        language,
        stack,
        package_manager,
        docker,
        git,
        tests,
        cuda,
    )
}
