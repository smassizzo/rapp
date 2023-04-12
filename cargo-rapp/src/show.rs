use cargo_metadata::{camino::Utf8PathBuf, DependencyKind, MetadataCommand, Package};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Show;

impl Show {
    pub fn run(&mut self) {
        let dir = scratch::path("rapp_runner");

        // Get saved config or else create it
        let mut config = Config::read_from(&dir);
        if config.is_none() {
            config = create_and_save(&dir);
        }

        if let Some(config) = config {
            println!("{config:#?}");
        } else {
            println!("Could not find or select a rapp project in current dir. Please change the dir of provide the path of the project");
        }

        // println!("Run cargo build - with target dir");
        // println!("set env var RAPP_RUNNER_STOP to false");
        // println!("Run cargo build - with target dir");
        // println!("Run cargo run - with target dir");
        // println!("set env var RAPP_RUNNER_STOP to false");
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct Config {
    name: String,
    target_dir: Utf8PathBuf,
}

impl Config {
    fn read_from(dir: &Path) -> Option<Self> {
        println!("Try read existing config from temp dir");

        // Create full path
        let path = Self::full_path(dir);

        // Read from file
        fs::read_to_string(path)
            .ok()
            .and_then(|s| ron::from_str::<Config>(&s).ok())
    }

    fn write_to(&self, dir: &Path) {
        println!("Try write config to temp dir");

        // Create full path
        let path = Self::full_path(dir);

        // Write to file
        let _ = ron::to_string(&self).map(|s| fs::write(&path, s));
    }

    fn full_path(dir: &Path) -> PathBuf {
        let mut path = dir.to_path_buf();
        path.push("config");
        path
    }
}

fn create_and_save(dir: &Path) -> Option<Config> {
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
            println!("None found");
            return None;
        }
        1 => packages_depending_on_rap.first().unwrap(), // package found
        _ => {
            println!(
                "Multiple candiates found in workspace: {:?}. Provide the name with option '--name <name>'",
                packages_depending_on_rap
                    .iter()
                    .map(|p| p.name.clone())
                    .collect::<Vec<String>>()
            );
            println!("{:?}", packages_depending_on_rap.get(0).unwrap());
            return None;
        }
    };

    let config = Config {
        name: package.name.clone(),
        target_dir: meta.target_directory,
    };

    // Save
    config.write_to(dir);

    Some(config)
}
