use cargo_rapp::{commands_parser::RappCmd, RappTool};
use clap::Parser;

// This is the cargo-rapp binary
fn main() {
    // It is run as "cargo rapp <rapp_subcommand>". Clap sees:
    // - 'cargo' as the first command
    // - 'rapp' as second command
    // - '<rapp_command>' as third command

    // Unpack the nested "cargo rapp <rapp_cmd>" structure
    let Cargo::Rapp(rapp_cmd) = Cargo::parse();

    RappTool.run(rapp_cmd);
}

#[derive(Parser, Debug)]
pub enum Cargo {
    #[clap(subcommand)]
    Rapp(RappCmd),
}
