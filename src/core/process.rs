use anyhow::{Result, anyhow};
use std::path::Path;
use std::process::Command;

pub fn run(program: &str, args: &[&str], workdir: &Path) -> Result<()> {
    let status = Command::new(program)
        .args(args)
        .current_dir(workdir)
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("command failed: {} {}", program, args.join(" ")))
    }
}
