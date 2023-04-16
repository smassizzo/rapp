pub mod cli_parser;
mod cmd_init;
mod cmd_show;
mod config;
mod error;
mod viewer;

use crate::cli_parser::RappCmd;
use cmd_init::Init;
use cmd_show::Show;
use log::LevelFilter;
use std::io::Write;

pub struct RappTool;

impl RappTool {
    pub fn run(&mut self, cmd: RappCmd) {
        env_logger::builder()
            .filter_level(LevelFilter::Debug)
            .format(|buf, record| writeln!(buf, "- {}", record.args()))
            .init();

        let result = match cmd {
            RappCmd::Init { path: name } => {
                Init { path: name }.run();
                Ok(())
            }
            RappCmd::Show {
                rebuild,
                use_relative_paths: use_local_paths,
            } => Show {
                rebuild,
                use_relative_paths: use_local_paths,
            }
            .run(),
        };

        if let Err(err) = result {
            println!("cargo:warning={}", err);
        }
    }
}
