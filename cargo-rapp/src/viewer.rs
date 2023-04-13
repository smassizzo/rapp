use std::{
    fs::{self, ReadDir},
    path::PathBuf,
};

use anyhow::Result;
use log::debug;

use crate::config::Config;

#[derive(Debug)]
pub(crate) struct Viewer<'a> {
    _config: &'a Config,
    dir: PathBuf,
}

impl<'a> Viewer<'a> {
    pub(crate) fn load_or_create(config: &'a Config, dir: PathBuf) -> Result<Self> {
        let mut viewer = Self {
            _config: config,
            dir,
        };
        if !viewer.exits()? {
            debug!("Generate code for runner");
            viewer.create()?
        } else {
            debug!("Use existing code for runner");
        }
        Ok(viewer)
    }

    fn exits(&mut self) -> Result<bool> {
        let scr_dir = self.src_dir()?;
        let mut read_dir = fs::read_dir(scr_dir)?;
        Ok(contains_file(&mut read_dir, "Cargo.toml"))
    }

    fn create(&mut self) -> Result<()> {
        let cargo_toml = include_bytes!("../code_gen/Cargo.toml");
        let mut path = self.src_dir()?;
        path.push("Cargo.toml");
        fs::write(path, cargo_toml)?;
        Ok(())
    }

    fn src_dir(&self) -> Result<PathBuf> {
        let mut path = self.dir.clone();
        path.push("src");
        if !path.exists() {
            fs::create_dir(&path)?;
        }
        Ok(path)
    }
}

fn contains_file(read_dir: &mut ReadDir, name: &str) -> bool {
    read_dir.any(|entry| entry.is_ok() && entry.unwrap().file_name() == name)
}
