use clap::Parser;

#[derive(Parser, Debug)]
pub enum RappCmd {
    /// Create a new rapp project
    Init {
        /// Path of the project. Defaults to .
        path: Option<String>,
    },
    /// Show the project and update on changes
    Show,
}
