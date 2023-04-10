use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct CargoRapp {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
    Show,
}

#[derive(Debug, Parser)]
#[clap(name = "cargo-rapp", bin_name = "cargo", version)]
enum Cargo {
    #[clap(alias = "rapp")]
    Rapp(CargoRapp),
}

fn main() {
    let Cargo::Rapp(cmd) = Cargo::parse();

    match &cmd.command {
        Commands::Init => println!("init"),
        Commands::Show => println!("show"),
    }
}
