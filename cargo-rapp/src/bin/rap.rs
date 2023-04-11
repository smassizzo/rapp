use cargo_rapp::{clap_rapp_cmd::RappCmd as Rap, RappTool};
use clap::Parser;

// This is the stand-alone rap binary and is used as "rap <rapp_subcommand>"
fn main() {
    // Clap sees
    // - 'rap' as first command
    // - '<rapp_command>' as second command

    // Unpack the nested "cargo rapp <rapp_cmd>" structure
    let rapp_cmd = Rap::parse();

    RappTool.run(rapp_cmd);
}
