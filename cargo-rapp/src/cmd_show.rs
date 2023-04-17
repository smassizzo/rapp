use std::{env, path::PathBuf};

use crate::{config::Config, viewer::Viewer};
use anyhow::Result;
use log::{self, debug, info, trace};

pub struct Show {
    pub(crate) rebuild: Option<bool>,
    pub(crate) use_relative_paths: Option<bool>,
}

impl Show {
    pub fn run(&mut self) -> Result<()> {
        let cache_dir = create_cache_dir()?;
        debug!("Cache dir {cache_dir:#?}");

        let rebuild = Some(true) == self.rebuild;
        if rebuild {
            info!("Rebuild everyting");
        } else {
            debug!("Use data from previous run if available");
        }

        let use_relative_paths = Some(true) == self.use_relative_paths;
        if use_relative_paths {
            info!("Use relative path to the rapp and runner crates in the current workspace");
        }

        // Get saved config or else create it
        let mut config = match Config::read_from(&cache_dir) {
            Some(config) if { !rebuild } => {
                debug!("Re-use config from previous run");
                config
            }
            None | Some(_) => {
                debug!("Gather info and create config");
                Config::create_and_save(&cache_dir)?
            }
        };
        config.rebuild = rebuild;
        config.use_relative_paths = use_relative_paths;

        trace!("{:#?}", &config);

        // Get the viewer binary from cache. If it doesn't exist, generate and build it
        let viewer = Viewer::read_or_build(&config)?;

        // Run
        info!("Run {}", config.name);
        viewer.run()?;

        // println!("set env var RAPP_RUNNER_STOP to false");
        // println!("Run cargo build - with target dir");
        // println!("Run cargo run - with target dir");
        // println!("set env var RAPP_RUNNER_STOP to false");
        Ok(())
    }
}

fn create_cache_dir() -> Result<PathBuf> {
    let mut dir = env::current_dir()?;

    dir.push("target");
    if !&dir.exists() {
        std::fs::create_dir(&dir)?;
    }

    dir.push("debug");
    if !&dir.exists() {
        std::fs::create_dir(&dir)?;
    }

    dir.push("build");
    if !&dir.exists() {
        std::fs::create_dir(&dir)?;
    }

    dir.push("rapp_runner");
    if !&dir.exists() {
        std::fs::create_dir(&dir)?;
    }

    Ok(dir)
}
