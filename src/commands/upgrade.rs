use anyhow::{Result, Context};

pub fn run() -> Result<()> {
    let release = self_update::backends::github::Update::configure()
        .repo_owner("mackin5150")
        .repo_name("spaces")
        .bin_name("spaces")
        .target(self_update::get_target())
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .no_confirm(true)
        .build()
        .context("Failed to check for updates")?;

    let status = release.update().context("Update failed")?;

    match status {
        self_update::Status::UpToDate(version) => {
            println!("Already latest version: {}", version);
        }
        self_update::Status::Updated(version) => {
            println!("Updated to version {} ✓", version);
        }
    }

    Ok(())
}
