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

    println!("\nTIP: Omit the path argument to scaffold the workspace in your current directory.");
    println!("  Example: spaces hf         -> installs HuggingFace workspace here");
    println!("           spaces hf my-ai   -> installs into ./my-ai/\n");

    println!("Shortcut commands and abbreviations:");
    println!("  python      aliases: py, pyspace, pyworkspace");
    println!("  node        aliases: njs, nodespace");
    println!("  go          aliases: goworkspace, gospace");
    println!("  lua         aliases: luaworkspace, luaspace");
    println!("  hardhat     alias: hh");
    println!("  express     alias: exp");
    println!("  flask       alias: fl");
    println!("  django      alias: dj");
    println!("  fastapi     alias: fapi");
    println!("  huggingface alias: hf");
    println!("  vite        (already short)");
    println!("  nextjs      aliases: nx, next");
    println!("  sveltekit   aliases: sk, sv");
    println!("  llama-cpp   aliases: llamacpp, llm\n");

    Ok(())
}