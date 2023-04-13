use crate::error::Error;
use anyhow::bail;
use anyhow::Result;
use cargo_metadata::{camino::Utf8PathBuf, DependencyKind, MetadataCommand, Package};
use log::{self, debug};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Show;

impl Show {
    pub fn run(&mut self) -> Result<()> {
        let temp_scr_dir = scratch::path("rapp_runner");
        debug!("Use temp dir: {temp_scr_dir:#?}");

        // Get saved config or else create it
        let config = match Config::read_from(&temp_scr_dir) {
            Some(config) => {
                debug!("Use cached config");
                config
            }
            None => {
                debug!("Create and save new config");
                create_and_save(&temp_scr_dir)?
            }
        };

        println!("{config:#?}");

        // println!("Run cargo build - with target dir");
        // println!("set env var RAPP_RUNNER_STOP to false");
        // println!("Run cargo build - with target dir");
        // println!("Run cargo run - with target dir");
        // println!("set env var RAPP_RUNNER_STOP to false");
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Config {
    name: String,
    target_dir: Utf8PathBuf,
}

impl Config {
    fn read_from(dir: &Path) -> Option<Self> {
        let path = Self::full_path(dir);
        fs::read_to_string(path)
            .ok()
            .and_then(|s| ron::from_str::<Config>(&s).ok())
    }

    fn write_to(&self, dir: &Path) -> anyhow::Result<()> {
        let path = Self::full_path(dir);
        fs::write(path, ron::to_string(&self)?)?;
        Ok(())
    }

    fn full_path(dir: &Path) -> PathBuf {
        let mut path = dir.to_path_buf();
        path.push("config");
        path
    }
}

fn create_and_save(dir: &Path) -> Result<Config> {
    println!("Create config");
    let meta = MetadataCommand::default().exec().unwrap();
    let mut packages_depending_on_rap: Vec<Package> = vec![];

    // collect packages that have dependency on rapp
    for package_id in meta.workspace_members.clone() {
        for package in &meta.packages {
            if package.targets.iter().any(|t| !t.is_lib()) {
                // not a lib
                continue;
            }
            // match by name
            let name = package_id.repr.split(' ').next().unwrap();
            if package
                .dependencies
                .iter()
                .any(|d| d.name == name && d.kind == DependencyKind::Normal)
            {
                // this package:
                // - has the same name as the workspace memnber,
                // - is a lib,
                // - depends on rapp
                packages_depending_on_rap.push(package.clone());
            }
        }
    }

    // depublicate
    packages_depending_on_rap.sort_by(|a, b| a.name.cmp(&b.name));
    packages_depending_on_rap.dedup_by(|a, b| a.name.eq(&b.name));

    let package = match packages_depending_on_rap.len() {
        0 => {
            bail!(Error::NoRappCrateFound(dir.to_path_buf()));
        }
        1 => packages_depending_on_rap.first().unwrap(), // package found
        _ => {
            let names = packages_depending_on_rap
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<String>>();

            bail!(Error::MultipleRappCratesFound(names));
        }
    };

    let config = Config {
        name: package.name.clone(),
        target_dir: meta.target_directory,
    };

    // Save
    config.write_to(dir)?;

    Ok(config)
}
