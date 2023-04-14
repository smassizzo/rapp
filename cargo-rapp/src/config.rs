use crate::error::RappError;
use anyhow::{bail, Result};
use cargo_metadata::{camino::Utf8PathBuf, DependencyKind, MetadataCommand, Package};

use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct Config {
    pub(crate) name: String,
    pub(crate) target_dir: Utf8PathBuf,
    pub(crate) scratch_dir: PathBuf,
}

impl Config {
    pub(crate) fn read_from(dir: &Path) -> Option<Self> {
        let path = Self::config_file_path(dir);
        fs::read_to_string(path)
            .ok()
            .and_then(|s| ron::from_str::<Config>(&s).ok())
    }

    pub(crate) fn create_and_save(dir: &Path) -> Result<Config> {
        let meta = MetadataCommand::default().exec().unwrap();
        let rapp_candidates = rapp_candidates(&meta);

        if rapp_candidates.is_empty() {
            bail!(RappError::NoRappCrateFound(dir.to_path_buf()));
        }

        if rapp_candidates.len() > 1 {
            let names = rapp_candidates
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<String>>();

            bail!(RappError::MultipleRappCratesFound(names));
        }

        // There is one candiate
        let candidate = rapp_candidates.first().ok_or(RappError::Other(
            "Expected one candidate, but none were found".to_string(),
        ))?;

        let config = Config {
            name: candidate.name.clone(),
            target_dir: meta.target_directory,
            scratch_dir: dir.to_path_buf(),
        };

        // Save
        config.write_to(dir)?;

        Ok(config)
    }

    fn write_to(&self, dir: &Path) -> anyhow::Result<()> {
        let file = Self::config_file_path(dir);
        fs::write(file, ron::to_string(&self)?)?;
        Ok(())
    }

    fn config_file_path(dir: &Path) -> PathBuf {
        let mut path = dir.to_path_buf();
        path.push("config");
        path
    }
}

fn rapp_candidates(meta: &cargo_metadata::Metadata) -> Vec<Package> {
    let mut packages_depending_on_rap: Vec<Package> = vec![];

    // collect all workspace members that have a dependency on rapp and are a lib
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
                // - is a workspace memnber,
                // - is a lib,
                // - depends on rapp
                packages_depending_on_rap.push(package.clone());
            }
        }
    }

    // depublicate
    packages_depending_on_rap.sort_by(|a, b| a.name.cmp(&b.name));
    packages_depending_on_rap.dedup_by(|a, b| a.name.eq(&b.name));

    packages_depending_on_rap
}
