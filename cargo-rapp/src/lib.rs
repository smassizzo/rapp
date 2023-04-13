pub mod clap_rapp_cmd;
mod error;
mod init;
mod show;

use crate::clap_rapp_cmd::RappCmd;
use init::Init;
use log::LevelFilter;
use show::Show;

pub struct RappTool;

impl RappTool {
    pub fn run(&mut self, cmd: RappCmd) {
        env_logger::init();
        log::set_max_level(LevelFilter::Debug);

        let result = match cmd {
            RappCmd::Init { path: name } => {
                Init { path: name }.run();
                Ok(())
            }
            RappCmd::Show => Show.run(),
        };

        if let Err(err) = result {
            println!("cargo:warning={}", err);
        }
    }
}
