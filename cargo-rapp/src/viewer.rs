use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Result};
use log::debug;
use serde::{Deserialize, Serialize};

use crate::{config::Config, error::RappError};

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct Viewer {
    bin: Option<PathBuf>,
    cache_dir: PathBuf,
    target_dir: PathBuf,
}

impl Viewer {
    pub(crate) fn read_or_build(config: &Config) -> Result<Self> {
        if let Some(viewer) = Viewer::read_from(&config.scratch_dir) {
            debug!("Reuse generated viewer from previous run");
            Ok(viewer)
        } else {
            debug!("Generate code for viewer");
            let viewer = Viewer::new(config)?;

            viewer.write_to(&config.scratch_dir)?;

            Ok(viewer)
        }
    }

    pub(crate) fn run(&self) -> Result<()> {
        let bin = if let Some(bin) = &self.bin {
            if !bin.exists() {
                bail!(RappError::Other(format!(
                    "Viewer binary not found at {:#?}",
                    bin
                )));
            } else {
                bin
            }
        } else {
            bail!(RappError::Other(
                "Viewer has not yet been build".to_string()
            ));
        };

        // Run
        let output = Command::new(bin).output()?;
        if output.status.success() {
            println!("{}", String::from_utf8(output.stdout)?);
        } else {
            dbg!(output);
        }
        Ok(())
    }

    fn build(&mut self) -> Result<()> {
        let mut build_sh = PathBuf::new();
        build_sh.push(&self.cache_dir);
        build_sh.push("build.sh");
        if !build_sh.exists() {
            bail!(RappError::Other(format!("File not found: {:#?}", build_sh)))
        }

        debug!("Build viewer");
        let output = Command::new("chmod").arg("+x").arg(&build_sh).output()?;
        if !output.status.success() {
            println!("{}", output.status);
            println!("{}", String::from_utf8(output.clone().stdout)?);
            dbg!(output);
        }

        let output = Command::new(&build_sh).output()?;
        if !output.status.success() {
            println!("{}", output.status);
            println!("{}", String::from_utf8(output.clone().stdout)?);
            dbg!(output);
        }

        let mut bin = PathBuf::new();
        bin.push(&self.target_dir);
        bin.push("debug");
        bin.push("rapp_runner");
        if bin.exists() {
            self.bin = Some(bin);
            Ok(())
        } else {
            bail!(RappError::Other(format!(
                "Build did not generate a binary at {bin:#?}"
            )));
        }
    }

    fn new(config: &Config) -> Result<Viewer> {
        let mut viewer = Viewer {
            bin: None,
            cache_dir: config.scratch_dir.clone(),
            target_dir: config.target_dir.clone().into(),
        };

        // Save Cargo.toml
        let mut cargo_toml = viewer.cache_dir.clone();
        cargo_toml.push("Cargo.toml");
        fs::write(cargo_toml, include_bytes!("../code_gen/Cargo.toml"))?;

        // Save build.sh
        let mut build_sh = viewer.cache_dir.clone();
        build_sh.push("build.sh");
        fs::write(build_sh, include_bytes!("../code_gen/build.sh"))?;

        // Create src dir (to place main.rs)
        let mut src_dir = viewer.cache_dir.clone();
        src_dir.push("src");
        if !src_dir.exists() {
            fs::create_dir(&src_dir)?;
        }

        // Save main.rs
        let mut main_rs = src_dir;
        main_rs.push("main.rs");
        fs::write(main_rs, include_bytes!("../code_gen/src/main.rs"))?;

        // Build
        viewer.build()?;

        Ok(viewer)
    }

    pub(crate) fn read_from(dir: &Path) -> Option<Self> {
        let file = Self::viewer_file_path(dir);
        fs::read_to_string(file)
            .ok()
            .and_then(|s| ron::from_str::<Viewer>(&s).ok())
    }

    fn write_to(&self, dir: &Path) -> Result<()> {
        let file = Self::viewer_file_path(dir);
        fs::write(file, ron::to_string(&self)?)?;
        Ok(())
    }

    fn viewer_file_path(dir: &Path) -> PathBuf {
        let mut path = dir.to_path_buf();
        path.push("viewer");
        path
    }
}

// fn contains_file(read_dir: &mut ReadDir, name: &str) -> bool {
//     read_dir.any(|entry| entry.is_ok() && entry.unwrap().file_name() == name)
// }
