pub mod clap_rapp_cmd;
//mod error;
mod init;
mod show;

use crate::clap_rapp_cmd::RappCmd;
use init::Init;
use show::Show;

pub struct RappTool;

impl RappTool {
    pub fn run(&mut self, cmd: RappCmd) {
        match cmd {
            RappCmd::Init { path: name } => Init { path: name }.run(),
            RappCmd::Show => Show.run(),
        }
    }
}
