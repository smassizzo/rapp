use std::{env, path::PathBuf};

use crate::{config::Config, viewer::Viewer};
use anyhow::Result;
use log::{self, debug};

pub struct Show;

impl Show {
    pub fn run(&mut self) -> Result<()> {
        let cache_dir = create_cache_dir()?; //scratch::path("rapp_runner");
        debug!("Cache dir {cache_dir:#?}");

        // Get saved config or else create it
        let config = match Config::read_from(&cache_dir) {
            Some(config) => {
                debug!("Reuse config from previous run");
                config
            }
            None => {
                debug!("Gather info and create config");
                Config::create_and_save(&cache_dir)?
            }
        };

        // Get the viewer binary. If it doesn't exist, generate it
        let viewer = Viewer::read_or_build(&config)?;

        // Run
        debug!("Show '{}' in viewer", config.name);
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
