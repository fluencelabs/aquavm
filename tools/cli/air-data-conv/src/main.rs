use std::{path::{PathBuf, Path}, io::{Read, Write}, fs::File};

use air_interpreter_data::InterpreterData;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    output: PathBuf,

}

fn main() {
    let args = Args::parse();

    convert_to_bin(std::io::stdin(), &args.output).unwrap();
    println!("Hello, world!");
}

fn convert_to_bin<R: Read>(mut inp: R, output: &Path) -> Result<(), std::io::Error> {
    use air_interpreter_interface::InterpreterOutcome;
    use rkyv::Serialize;

    let particle: InterpreterData  = serde_json::from_reader(inp).unwrap();
    dbg!(&particle);

    let mut ser = rkyv::ser::serializers::AllocSerializer::<1024>::new(<_>::default());
    particle.serialize(&mut ser);

    let mut out = File::create(output)?;
    out.write_all(&ser.into_inner())?;
    Ok(())
}
