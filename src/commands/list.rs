use crate::workspace::manager::adapters;
use anyhow::Result;

pub fn run() -> Result<()> {
    println!("Available workspace combinations:\n");

    for adapter in adapters() {
        println!(
            "{:<8} {:<12} {:<6} {}",
            adapter.language().as_str(),
            adapter.stack().as_str(),
            adapter.package_manager().as_str(),
            adapter.description()
        );
    }

    println!("\nShortcut commands:");
    println!("python       alias: pyworkspace");
    println!("hardhat      node hardhat preset");
    println!("express      node express preset");
    println!("fastapi      python fastapi preset");
    println!("huggingface  python huggingface preset");
    println!("vite         node vite preset");
    println!("llama-cpp    cpp llama.cpp preset");
    println!("go           alias: goworkspace");
    println!("lua          alias: luaworkspace");

    Ok(())
}
