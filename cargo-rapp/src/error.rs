use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("given path {0} is not a folder")]
    RootNotDir(String),
    #[error("unable to initialize workspace: {0}")]
    Init(String),
}
