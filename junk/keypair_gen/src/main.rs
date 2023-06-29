use air_test_utils::key_utils::derive_dummy_keypair;
use clap::{Parser, Subcommand};
use fluence_keypair::KeyPair;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Debug, Subcommand)]
enum Mode {
    Random,
    Name { name: String },
}

fn main() {
    let args = Cli::parse();
    let keypair = match args.mode {
        Mode::Random => KeyPair::generate_ed25519(),
        Mode::Name { name } => derive_dummy_keypair(&name).0,
    };
    let keyp = keypair.to_vec();
    let keyp58 = bs58::encode(&keyp).into_string();
    println!("{keyp58}");
}
