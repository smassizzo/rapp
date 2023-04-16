use clap::Parser;

#[derive(Parser, Debug)]
pub enum RappCmd {
    /// Create a new rapp project
    Init {
        /// Path of the project. Defaults to .
        path: Option<String>,
    },
    /// Show the project and update on changes
    Show {
        /// When set to true everyting is rebuild, otherwise chached data is used
        rebuild: Option<bool>,

        /// Use in rapp wor paths to rapp and runner are used. This option is used by the Rapp library developers for test and CI. Don't use if you are developing a regular app.
        use_relative_paths: Option<bool>,
    },
}
