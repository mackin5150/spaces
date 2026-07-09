use crate::models::workspace_request::{Language, PackageManager, Stack};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "spaces")]
#[command(version = "0.1.0")]
#[command(about = "Universal tuned developer workspaces — omit the path to scaffold in the current directory")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    New {
        #[arg(default_value = ".")]
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
        install: bool,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
        #[arg(long)]
        cuda: bool,
    },
    #[command(alias = "pyworkspace", alias = "py", alias = "pyspace")]
    Python {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum, default_value_t = Stack::Basic)]
        stack: Stack,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        install: bool,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
        #[arg(long)]
        cuda: bool,
    },
    #[command(alias = "njs", alias = "nodespace")]
    Node {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum, default_value_t = Stack::Hardhat)]
        stack: Stack,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        install: bool,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
    },
    #[command(alias = "goworkspace", alias = "gospace")]
    Go {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum, default_value_t = Stack::Basic)]
        stack: Stack,
        #[arg(long)]
        install: bool,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
    },
    #[command(alias = "luaworkspace", alias = "luaspace")]
    Lua {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum, default_value_t = Stack::Basic)]
        stack: Stack,
        #[arg(long)]
        install: bool,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
    },
    #[command(alias = "hh")]
    Hardhat {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        install: bool,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
    },
    #[command(alias = "exp")]
    Express {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        install: bool,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
    },
    #[command(alias = "fl")]
    Flask {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        install: bool,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
    },
    #[command(alias = "dj")]
    Django {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        install: bool,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
    },
    #[command(alias = "fapi")]
    Fastapi {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        install: bool,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
        #[arg(long)]
        tests: bool,
    },
    #[command(alias = "hf")]
    Huggingface {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        install: bool,
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
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        install: bool,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
    },
    #[command(alias = "nx", alias = "next")]
    Nextjs {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        install: bool,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
    },
    #[command(alias = "sk", alias = "sv")]
    Sveltekit {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long, value_enum)]
        pm: Option<PackageManager>,
        #[arg(long)]
        install: bool,
        #[arg(long)]
        docker: bool,
        #[arg(long)]
        git: bool,
    },
    #[command(name = "llama-cpp", alias = "llamacpp", alias = "llm")]
    LlamaCpp {
        #[arg(default_value = ".")]
        path: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        install: bool,
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