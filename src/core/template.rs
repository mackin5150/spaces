use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub type TemplateContext = HashMap<&'static str, String>;

pub fn render_template_dir(
    source: &Path,
    destination: &Path,
    context: &TemplateContext,
) -> Result<()> {
    if !source.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_path = entry.path();
        let target_path = destination.join(entry.file_name());

        if entry_path.is_dir() {
            fs::create_dir_all(&target_path)?;
            render_template_dir(&entry_path, &target_path, context)?;
            continue;
        }

        let contents = fs::read_to_string(&entry_path)?;
        fs::write(target_path, render_string(&contents, context))?;
    }

    Ok(())
}

pub fn template_root(parts: &[&str]) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("templates");
    for part in parts {
        path.push(part);
    }
    path
}

pub fn render_string(template: &str, context: &TemplateContext) -> String {
    let mut rendered = template.to_string();

    for (key, value) in context {
        let needle = format!("{{{{{}}}}}", key);
        rendered = rendered.replace(&needle, value);
    }

    rendered
}
