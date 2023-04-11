use cargo_rapp::{clap_rapp_cmd::RappCmd, RappTool};
use clap::Parser;

// This is the cargo-rap binary
fn main() {
    // It is run as "cargo rap <rapp_subcommand>". Clap sees:
    // - 'cargo' as the first command
    // - 'rapp' as second command
    // - '<rapp_command>' as third command

    // Unpack the nested "cargo rapp <rapp_cmd>" structure
    let Cargo::Rap(rapp_cmd) = Cargo::parse();

    RappTool.run(rapp_cmd);
}

#[derive(Parser, Debug)]
pub enum Cargo {
    #[clap(subcommand)]
    Rap(RappCmd),
}
