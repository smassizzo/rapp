use crate::error::RappError;
use anyhow::{bail, Result};
use cargo_metadata::{DependencyKind, MetadataCommand, Package};
use log::trace;
use serde::{Deserialize, Serialize};
use std::{
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Serialize, Default)]
pub(crate) struct Config {
    pub(crate) name: String,
    pub(crate) target_dir: PathBuf,
    pub(crate) scratch_dir: PathBuf,
    pub(crate) app_dir: PathBuf,
    pub(crate) use_relative_paths: bool,
    pub(crate) rebuild: bool,
}

impl Config {
    pub(crate) fn read_from(dir: &Path) -> Option<Self> {
        let path = Self::config_file_path(dir);
        fs::read_to_string(path)
            .ok()
            .and_then(|s| ron::from_str::<Config>(&s).ok())
    }

    pub(crate) fn create_and_save(cache_dir: &Path) -> Result<Config> {
        let app_dir = current_dir()?;

        let mut metadata_cmd = MetadataCommand::default();
        metadata_cmd.current_dir(&app_dir).no_deps();
        trace!("{:#?}", &metadata_cmd);
        let meta = metadata_cmd.exec()?;
        let rapp_candidates = rapp_candidates(&meta);

        if rapp_candidates.is_empty() {
            bail!(RappError::NoRappCrateFound(cache_dir.to_path_buf()));
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

        // Create target dir
        let mut target_dir = PathBuf::new();
        target_dir.push(meta.target_directory.as_str());
        if !target_dir.exists() {
            fs::create_dir(&target_dir)?;
        }
        target_dir.push("debug");
        if !target_dir.exists() {
            fs::create_dir(&target_dir)?;
        }
        target_dir.push("build");
        if !target_dir.exists() {
            fs::create_dir(&target_dir)?;
        }
        target_dir.push("rapp_runner");
        if !target_dir.exists() {
            fs::create_dir(&target_dir)?;
        }

        // Create config
        let config = Config {
            name: candidate.name.clone(),
            target_dir,
            scratch_dir: cache_dir.to_path_buf(),
            app_dir,
            ..Default::default()
        };

        // Save
        config.write_to(cache_dir)?;

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
        trace!("{package_id}");
        for package in &meta.packages {
            if package.targets.iter().any(|t| !t.is_lib()) {
                trace!("not a lib {}", package.name);
                // not a lib
                continue;
            }

            // filter special case for runner itself
            if package.name == "runner" {
                // when run in the main rapp workspace we don't want to conflict with the runner itself
                continue;
            }
            trace!("name {:#?}", package);
            if package
                .dependencies
                .iter()
                .any(|d| d.name == "rapp" && d.kind == DependencyKind::Normal)
            {
                // this package:
                // - is a workspace memnber,
                // - is a lib,
                // - depends on rapp

                trace!("add {}", package.name);

                packages_depending_on_rap.push(package.clone());
            }
        }
    }
    trace!("{:?}", &packages_depending_on_rap);

    // depublicate
    packages_depending_on_rap.sort_by(|a, b| a.name.cmp(&b.name));
    packages_depending_on_rap.dedup_by(|a, b| a.name.eq(&b.name));

    packages_depending_on_rap
}

#[cfg(test)]
mod tests {
    use crate::config::rapp_candidates;
    use cargo_metadata::Metadata;
    use log::LevelFilter;

    fn _init() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Trace)
            .try_init();
    }

    #[test]
    fn parse_metadata_1() {
        // init(); use if you want to see loggin
        let deserialized: Metadata = serde_json::from_str(METADATA_1).unwrap();
        let rapp_candidates = rapp_candidates(&deserialized);
        assert_eq!(rapp_candidates.len(), 1);
    }

    #[test]
    fn parse_metadata_2() {
        // init(); use if you want to see loggin
        let deserialized: Metadata = serde_json::from_str(METADATA_2).unwrap();
        let rapp_candidates = rapp_candidates(&deserialized);
        assert_eq!(rapp_candidates.len(), 1);
    }

    const METADATA_1: &str = r#"{
    "packages": [
        {
            "name": "my_rap",
            "version": "0.1.0",
            "id": "my_rap 0.1.0 (path+file:///Users/developer/Projects/my_rap/hello_app)",
            "license": null,
            "license_file": null,
            "description": null,
            "source": null,
            "dependencies": [
                {
                    "name": "rapp",
                    "source": "git+https://github.com/smassizzo/rapp.git",
                    "req": "*",
                    "kind": null,
                    "rename": null,
                    "optional": false,
                    "uses_default_features": true,
                    "features": [],
                    "target": null,
                    "registry": null
                }
            ],
            "targets": [
                {
                    "kind": [
                        "lib"
                    ],
                    "crate_types": [
                        "lib"
                    ],
                    "name": "my_rap",
                    "src_path": "/Users/developer/Projects/my_rap/hello_app/src/lib.rs",
                    "edition": "2021",
                    "doc": true,
                    "doctest": true,
                    "test": true
                }
            ],
            "features": {},
            "manifest_path": "/Users/developer/Projects/my_rap/hello_app/Cargo.toml",
            "metadata": null,
            "publish": null,
            "authors": [],
            "categories": [],
            "keywords": [],
            "readme": null,
            "repository": null,
            "homepage": null,
            "documentation": null,
            "edition": "2021",
            "links": null,
            "default_run": null,
            "rust_version": null
        },
        {
            "name": "rapp",
            "version": "0.1.0",
            "id": "rapp 0.1.0 (git+https://github.com/smassizzo/rapp.git#74bc0c52f6592913bec2809040351c574cc92a10)",
            "license": null,
            "license_file": null,
            "description": "A cargo tool to facilitate building mobile apps with rapp",
            "source": "git+https://github.com/smassizzo/rapp.git#74bc0c52f6592913bec2809040351c574cc92a10",
            "dependencies": [],
            "targets": [
                {
                    "kind": [
                        "lib"
                    ],
                    "crate_types": [
                        "lib"
                    ],
                    "name": "rapp",
                    "src_path": "/Users/developer/.cargo/git/checkouts/rapp-2dc35b25ab718a6e/74bc0c5/rapp/src/lib.rs",
                    "edition": "2021",
                    "doc": true,
                    "doctest": true,
                    "test": true
                }
            ],
            "features": {},
            "manifest_path": "/Users/developer/.cargo/git/checkouts/rapp-2dc35b25ab718a6e/74bc0c5/rapp/Cargo.toml",
            "metadata": null,
            "publish": null,
            "authors": [
                "Sebastiaan Massizzo"
            ],
            "categories": [],
            "keywords": [],
            "readme": null,
            "repository": null,
            "homepage": null,
            "documentation": null,
            "edition": "2021",
            "links": null,
            "default_run": null,
            "rust_version": "1.65"
        }
    ],
    "workspace_members": [
        "my_rap 0.1.0 (path+file:///Users/developer/Projects/my_rap/hello_app)"
    ],
    "resolve": {
        "nodes": [
            {
                "id": "my_rap 0.1.0 (path+file:///Users/developer/Projects/my_rap/hello_app)",
                "dependencies": [
                    "rapp 0.1.0 (git+https://github.com/smassizzo/rapp.git#74bc0c52f6592913bec2809040351c574cc92a10)"
                ],
                "deps": [
                    {
                        "name": "rapp",
                        "pkg": "rapp 0.1.0 (git+https://github.com/smassizzo/rapp.git#74bc0c52f6592913bec2809040351c574cc92a10)",
                        "dep_kinds": [
                            {
                                "kind": null,
                                "target": null
                            }
                        ]
                    }
                ],
                "features": []
            },
            {
                "id": "rapp 0.1.0 (git+https://github.com/smassizzo/rapp.git#74bc0c52f6592913bec2809040351c574cc92a10)",
                "dependencies": [],
                "deps": [],
                "features": []
            }
        ],
        "root": null
    },
    "target_directory": "/Users/developer/Projects/my_rap/target",
    "version": 1,
    "workspace_root": "/Users/developer/Projects/my_rap",
    "metadata": null
}"#;

    const METADATA_2: &str = r#"{
    "packages": [
        {
            "name": "hello_app",
            "version": "0.1.0",
            "id": "hello_app 0.1.0 (path+file:///Users/developer/Projects/my_rap/hello_app)",
            "license": null,
            "license_file": null,
            "description": null,
            "source": null,
            "dependencies": [
                {
                    "name": "rapp",
                    "source": "git+https://github.com/smassizzo/rapp.git",
                    "req": "*",
                    "kind": null,
                    "rename": null,
                    "optional": false,
                    "uses_default_features": true,
                    "features": [],
                    "target": null,
                    "registry": null
                }
            ],
            "targets": [
                {
                    "kind": [
                        "lib"
                    ],
                    "crate_types": [
                        "lib"
                    ],
                    "name": "hello_app",
                    "src_path": "/Users/developer/Projects/my_rap/hello_app/src/lib.rs",
                    "edition": "2021",
                    "doc": true,
                    "doctest": true,
                    "test": true
                }
            ],
            "features": {},
            "manifest_path": "/Users/developer/Projects/my_rap/hello_app/Cargo.toml",
            "metadata": null,
            "publish": null,
            "authors": [],
            "categories": [],
            "keywords": [],
            "readme": null,
            "repository": null,
            "homepage": null,
            "documentation": null,
            "edition": "2021",
            "links": null,
            "default_run": null,
            "rust_version": null
        },
        {
            "name": "rapp",
            "version": "0.1.0",
            "id": "rapp 0.1.0 (git+https://github.com/smassizzo/rapp.git#c525e19a788ec99d384bac61486c39bfddc5c6d5)",
            "license": null,
            "license_file": null,
            "description": "A cargo tool to facilitate building mobile apps with rapp",
            "source": "git+https://github.com/smassizzo/rapp.git#c525e19a788ec99d384bac61486c39bfddc5c6d5",
            "dependencies": [],
            "targets": [
                {
                    "kind": [
                        "lib"
                    ],
                    "crate_types": [
                        "lib"
                    ],
                    "name": "rapp",
                    "src_path": "/Users/developer/.cargo/git/checkouts/rapp-2dc35b25ab718a6e/c525e19/rapp/src/lib.rs",
                    "edition": "2021",
                    "doc": true,
                    "doctest": true,
                    "test": true
                }
            ],
            "features": {},
            "manifest_path": "/Users/developer/.cargo/git/checkouts/rapp-2dc35b25ab718a6e/c525e19/rapp/Cargo.toml",
            "metadata": null,
            "publish": null,
            "authors": [
                "Sebastiaan Massizzo"
            ],
            "categories": [],
            "keywords": [],
            "readme": null,
            "repository": null,
            "homepage": null,
            "documentation": null,
            "edition": "2021",
            "links": null,
            "default_run": null,
            "rust_version": "1.65"
        }
    ],
    "workspace_members": [
        "hello_app 0.1.0 (path+file:///Users/developer/Projects/my_rap/hello_app)"
    ],
    "resolve": {
        "nodes": [
            {
                "id": "hello_app 0.1.0 (path+file:///Users/developer/Projects/my_rap/hello_app)",
                "dependencies": [
                    "rapp 0.1.0 (git+https://github.com/smassizzo/rapp.git#c525e19a788ec99d384bac61486c39bfddc5c6d5)"
                ],
                "deps": [
                    {
                        "name": "rapp",
                        "pkg": "rapp 0.1.0 (git+https://github.com/smassizzo/rapp.git#c525e19a788ec99d384bac61486c39bfddc5c6d5)",
                        "dep_kinds": [
                            {
                                "kind": null,
                                "target": null
                            }
                        ]
                    }
                ],
                "features": []
            },
            {
                "id": "rapp 0.1.0 (git+https://github.com/smassizzo/rapp.git#c525e19a788ec99d384bac61486c39bfddc5c6d5)",
                "dependencies": [],
                "deps": [],
                "features": []
            }
        ],
        "root": null
    },
    "target_directory": "/Users/developer/Projects/my_rap/target",
    "version": 1,
    "workspace_root": "/Users/developer/Projects/my_rap",
    "metadata": null
}"#;
}
