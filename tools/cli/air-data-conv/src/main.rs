use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    rc::Rc,
};

use air_interpreter_data::{CallResult, CanonResult, ExecutedState, InterpreterData};
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    output: PathBuf,
    #[clap(long)]
    dedup: bool,
}

fn main() {
    let args = Args::parse();

    convert_to_bin(std::io::stdin(), &args.output, args.dedup).unwrap();
}

fn convert_to_bin<R: Read>(inp: R, output: &Path, dedup: bool) -> Result<(), std::io::Error> {
    let mut particle: InterpreterData = serde_json::from_reader(inp).unwrap();

    if dedup {
        use air_interpreter_data::ValueRef::*;

        // Dedup trace referenes
        for state in &mut *particle.trace {
            match state {
                ExecutedState::Call(CallResult::Executed(ex)) => match ex {
                    Scalar(ref mut cid) | Stream { ref mut cid, .. } => {
                        *cid = particle
                            .cid_info
                            .service_result_store
                            .get_key(cid)
                            .expect("Inconsistent data")
                            .clone();
                    }
                    Unused(_) => {}
                },
                ExecutedState::Canon(CanonResult::Executed(ref mut cid)) => {
                    *cid = particle
                        .cid_info
                        .canon_result_store
                        .get_key(cid)
                        .expect("Inconsistent data")
                        .clone();
                }
                _ => {}
            }
        }

        // Dedpup service_result_store reference
        for (_, service_result) in particle.cid_info.service_result_store.iter_mut() {
            let mutt = Rc::get_mut(service_result).unwrap();
            mutt.tetraplet_cid = particle
                .cid_info
                .tetraplet_store
                .get_key(&mutt.tetraplet_cid)
                .expect("Inconsistent data")
                .clone();
            mutt.value_cid = particle
                .cid_info
                .value_store
                .get_key(&mutt.value_cid)
                .expect("Inconsistent data")
                .clone();
        }

        // Dedup canon_result_store
        // todo!()
    }

    let mut ser = rkyv::ser::serializers::AllocSerializer::<1024>::default();
    rkyv::Serialize::serialize(&particle, &mut ser).unwrap();

    let mut out = File::create(output)?;
    out.write_all(&ser.into_serializer().into_inner()[..])?;
    Ok(())
}
