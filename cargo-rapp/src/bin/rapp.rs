use cargo_rapp::{commands_parser::RappCmd as Rapp, RappTool};
use clap::Parser;

// This is the stand-alone rapp binary and is used as "rapp <rapp_subcommand>"
fn main() {
    // Clap sees
    // - 'rapp' as first command
    // - '<rapp_command>' as second command

    // Unpack the nested "cargo rapp <rapp_cmd>" structure
    let rapp_cmd = Rapp::parse();

    RappTool.run(rapp_cmd);
}
