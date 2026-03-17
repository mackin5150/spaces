use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpacesError {
    #[error("unsupported workspace: {0}")]
    UnsupportedWorkspace(String),

    #[error("target directory already exists: {0}")]
    DirectoryAlreadyExists(String),

    #[error("missing workspace manifest: {0}")]
    MissingManifest(String),
}
