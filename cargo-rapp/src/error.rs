use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RappError {
    #[error("Could not find a lib crate with dependency on rapp in dir {0}")]
    NoRappCrateFound(PathBuf),

    #[error(
        "Multiple candidates found: {0:?}. Please indicate which one to show with option: --name <name>"
    )]
    MultipleRappCratesFound(Vec<String>),
}
