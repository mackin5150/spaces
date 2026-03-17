use crate::core::fs::write_file;
use crate::core::template::{TemplateContext, render_string, render_template_dir, template_root};
use crate::models::workspace_request::WorkspaceRequest;
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

pub fn base_context(request: &WorkspaceRequest) -> TemplateContext {
    let mut context = HashMap::new();
    context.insert("name", request.name.clone());
    context.insert(
        "package_manager",
        request.package_manager.as_str().to_string(),
    );
    context
}

pub fn render_stack_templates(
    root: &Path,
    request: &WorkspaceRequest,
    language: &str,
    stack: &str,
) -> Result<()> {
    let context = base_context(request);
    render_template_dir(&template_root(&[language, stack, "base"]), root, &context)?;

    if request.tests {
        render_template_dir(&template_root(&[language, stack, "tests"]), root, &context)?;
    }

    if request.docker {
        render_template_dir(&template_root(&[language, stack, "docker"]), root, &context)?;
    }

    if request.cuda {
        render_template_dir(&template_root(&[language, stack, "cuda"]), root, &context)?;
    }

    Ok(())
}

pub fn write_manifest(root: &Path, request: &WorkspaceRequest) -> Result<()> {
    let mut context = base_context(request);
    context.insert("language", request.language.as_str().to_string());
    context.insert("stack", request.stack.as_str().to_string());
    context.insert("features", render_features(request));

    let manifest = render_string(
        "name: {{name}}\nlanguage: {{language}}\nstack: {{stack}}\nversion: 2\npackage_manager: {{package_manager}}\ncreated_with: spaces 0.1.0\nfeatures:\n{{features}}\n",
        &context,
    );

    write_file(&root.join(".spaces/workspace.yaml"), &manifest)
}

fn render_features(request: &WorkspaceRequest) -> String {
    let features = request.feature_list();
    if features.is_empty() {
        "  []".to_string()
    } else {
        features
            .iter()
            .map(|feature| format!("  - {}", feature))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
