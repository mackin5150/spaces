use crate::models::workspace_request::{Language, PackageManager, Stack, WorkspaceRequest};
use anyhow::Result;
use std::path::Path;

pub trait WorkspaceAdapter {
    fn language(&self) -> Language;
    fn stack(&self) -> Stack;
    fn package_manager(&self) -> PackageManager;
    fn description(&self) -> &'static str;
    fn create(&self, root: &Path, request: &WorkspaceRequest) -> Result<()>;
    fn doctor(&self, root: &Path) -> Result<Vec<String>>;
}
