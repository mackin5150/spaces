use crate::models::workspace_request::{Language, PackageManager, Stack};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "spaces")]
#[command(version = "0.1.0")]
#[command(about = "Universal tuned developer workspaces")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    New {
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        lang: Language,
        #[arg(long, value_enum, default_value_t = Stack::Basic)]
        stack: Stack,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
        #[arg(long)]
        cuda: bool,
    },
    #[command(alias = "pyworkspace")]
    Python {
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum, default_value_t = Stack::Basic)]
        stack: Stack,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
        #[arg(long)]
        cuda: bool,
    },
    Node {
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum, default_value_t = Stack::Hardhat)]
        stack: Stack,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
    },
    #[command(alias = "goworkspace")]
    Go {
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum, default_value_t = Stack::Basic)]
        stack: Stack,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
    },
    #[command(alias = "luaworkspace")]
    Lua {
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum, default_value_t = Stack::Basic)]
        stack: Stack,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
    },
    Hardhat {
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
    },
    Express {
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
    },
    Fastapi {
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
    },
    Huggingface {
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
        #[arg(long)]
        cuda: bool,
    },
    Vite {
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
    },
    #[command(name = "llama-cpp", alias = "llamacpp")]
    LlamaCpp {
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        cuda: bool,
    },
    List,
    Doctor,
}
