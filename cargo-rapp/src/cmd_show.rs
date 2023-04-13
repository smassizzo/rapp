use crate::{config::Config, viewer::Viewer};
use anyhow::Result;
use log::{self, debug};

pub struct Show;

impl Show {
    pub fn run(&mut self) -> Result<()> {
        let temp_scr_dir = scratch::path("rapp_runner");
        debug!("Dir used for caching and code generation: {temp_scr_dir:#?}");

        // Get saved config or else create it
        let config = match Config::read_from(&temp_scr_dir) {
            Some(config) => {
                debug!("Reuse config from previous run");
                config
            }
            None => {
                debug!("Create and save new config");
                Config::create_and_save(&temp_scr_dir)?
            }
        };

        debug!("{config:#?}");

        let viewer = Viewer::load_or_create(&config, temp_scr_dir);

        // println!("Run cargo build - with target dir");
        // println!("set env var RAPP_RUNNER_STOP to false");
        // println!("Run cargo build - with target dir");
        // println!("Run cargo run - with target dir");
        // println!("set env var RAPP_RUNNER_STOP to false");
        Ok(())
    }
}
