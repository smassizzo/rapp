pub mod clap_rapp_cmd;
mod error;
mod init;
mod show;

use crate::clap_rapp_cmd::RappCmd;
use init::Init;
use log::LevelFilter;
use show::Show;
use std::io::Write;
pub struct RappTool;

impl RappTool {
    pub fn run(&mut self, cmd: RappCmd) {
        env_logger::builder()
            .filter_level(LevelFilter::Debug)
            .format(|buf, record| writeln!(buf, "{}", record.args()))
            .init();

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
