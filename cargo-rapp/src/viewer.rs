use crate::{config::Config, error::RappError};
use anyhow::{bail, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct Viewer {
    bin: Option<PathBuf>,
    cache_dir: PathBuf,
    target_dir: PathBuf,
    use_relative_paths: bool,
}

impl Viewer {
    pub(crate) fn read_or_build(config: &Config) -> Result<Self> {
        if !config.rebuild {
            if let Some(viewer) = Viewer::read_from(&config.scratch_dir) {
                debug!("Re-use generated viewer from previous run");
                return Ok(viewer);
            } else {
                debug!("No viewer from previous run in cache");
            }
        }

        debug!("Generate code for viewer");

        let viewer = Viewer::new(config)?;

        viewer.write_to(&config.scratch_dir)?;

        Ok(viewer)
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
        let mut bin = PathBuf::new();
        bin.push(&self.target_dir);
        bin.push("target");
        bin.push("debug");
        bin.push("rapp_runner");

        // remove old binary
        if bin.exists() {
            fs::remove_file(&bin)?;
        }

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

        // Check if everyting went well
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
            target_dir: config.target_dir.clone(),
            use_relative_paths: config.use_relative_paths,
        };
        debug!("Viewer before building: \n {:#?}", &viewer);

        // Save Cargo.toml
        let mut cargo_toml = viewer.cache_dir.clone();
        cargo_toml.push("Cargo.toml");

        let app_dir = config.app_dir.to_str().ok_or(RappError::Other(
            "could not convert path to string".to_string(),
        ))?;

        let mut cargo_toml_content = if viewer.use_relative_paths {
            String::from_utf8(include_bytes!("../code_gen/CargoRelativePaths.toml").to_vec())?
        } else {
            String::from_utf8(include_bytes!("../code_gen/Cargo.toml").to_vec())?
        };
        cargo_toml_content = cargo_toml_content.replace("${dir}", app_dir);
        cargo_toml_content = cargo_toml_content.replace("${name}", &config.name);

        fs::write(cargo_toml, cargo_toml_content)?;

        // Create build.sh content
        let mut build_sh_content = String::new();
        build_sh_content.push_str("cd ");
        build_sh_content.push_str(format!("{:?}\n", viewer.cache_dir).as_str());
        build_sh_content.push_str("cargo build");

        // Save build.sh
        let mut build_sh = viewer.cache_dir.clone();
        build_sh.push("build.sh");
        fs::write(build_sh, build_sh_content)?;

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

        debug!(
            "Viewer after building. The bin should be set by now: \n {:#?}",
            &viewer
        );

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
