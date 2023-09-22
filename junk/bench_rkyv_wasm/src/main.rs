use air_interpreter_data::InterpreterData;
use bencher::*;
use std::hint::black_box;


/*
 * TODO:
 *
 * 1. deserialize
 *    a. serde_json
 *    b. simd-json
 *    c. rkyv (even if it is intended to be used in archived form)
 * 2. serialize
 *    a. serde_json
 *    b. simd-json
 *    c. rkyv
 *
 *
 * DATA SIZE:
 * | rkyv    |   717,696
 * | borsh   | 1,045,375
 * | json    | 1,267,245
 * | cborium | 1,277,966
 *
 */

// 1267245
const DATA_STR: &str =
    include_str!("../../../benches/performance_metering/multiple-cids50/cur_data.json");

fn deserialize_serde_json(bencher: &mut Bencher) {
    bencher.iter(|| serde_json::from_str::<InterpreterData>(black_box(DATA_STR)));
}

fn deserialize_serde_json2(bencher: &mut Bencher) {
    bencher.iter(|| {
        let data = black_box(DATA_STR).to_owned();
        serde_json::from_str::<InterpreterData>(&data).unwrap()
    });
}

fn deserialize_simd_json_invalid(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut data = black_box(DATA_STR).as_bytes().to_owned();
        // TODO INVALID!!!
        simd_json::serde::from_slice::<InterpreterData>(&mut data)
    });
}

fn deserialize_ciborium(bencher: &mut Bencher) {
    let data: InterpreterData = serde_json::from_str(DATA_STR).unwrap();
    let mut buf = vec![];
    ciborium::ser::into_writer(black_box(&data), &mut buf).unwrap();
    bencher.iter(|| {
        ciborium::de::from_reader::<InterpreterData, _>(black_box(buf.as_slice())).unwrap()
    });
}

fn deserialize_rkyv_dedup_validate_only(bencher: &mut Bencher) {
    use rkyv::ser::Serializer;

    let data: InterpreterData = serde_json::from_str(DATA_STR).unwrap();
    let data = dedup(data);
    let mut ser = rkyv::ser::serializers::AllocSerializer::<4096>::default();
    ser.serialize_value(&data).unwrap();

    let buf = ser.into_serializer().into_inner();
    let _root = rkyv::check_archived_root::<InterpreterData>(&buf).expect("precheck broken");

    bencher.iter(|| {
        let buf = black_box(&buf);
        let root = rkyv::check_archived_root::<InterpreterData>(buf).unwrap();
        root
    });
}

fn deserialize_rkyv_dedup(bencher: &mut Bencher) {
    use air_interpreter_data::InterpreterDataDeserializer;
    use rkyv::ser::Serializer;

    let data: InterpreterData = serde_json::from_str(DATA_STR).unwrap();
    let data = dedup(data);
    let mut ser = rkyv::ser::serializers::AllocSerializer::<4096>::default();
    ser.serialize_value(&data).unwrap();
    let buf = ser.into_serializer().into_inner();

    bencher.iter(|| {
        let buf = black_box(&buf);
        let root = rkyv::check_archived_root::<InterpreterData>(buf).unwrap();

        let mut des = InterpreterDataDeserializer::default();
        rkyv::Deserialize::<InterpreterData, _>::deserialize(root, &mut des).unwrap()
    });
}

fn serialize_serde_json(bencher: &mut Bencher) {
    let data: InterpreterData = serde_json::from_str(DATA_STR).unwrap();
    bencher.iter(|| serde_json::to_vec(black_box(&data)).unwrap());
}

fn serialize_simd_json(bencher: &mut Bencher) {
    let data: InterpreterData = serde_json::from_str(DATA_STR).unwrap();
    bencher.iter(|| simd_json::serde::to_vec(black_box(&data)).unwrap());
}

fn serialize_ciborium(bencher: &mut Bencher) {
    let data: InterpreterData = serde_json::from_str(DATA_STR).unwrap();
    // 1277966
    // {
    //     let mut buf = vec![];
    //     ciborium::ser::into_writer(black_box(&data), &mut buf).unwrap();
    //     eprintln!("cborium size: {}", buf.len());
    // }
    bencher.iter(|| {
        let mut buf = vec![];
        ciborium::ser::into_writer(black_box(&data), &mut buf).unwrap();
        buf
    });
}

fn serialize_borshium(bencher: &mut Bencher) {
    let data: InterpreterData = serde_json::from_str(DATA_STR).unwrap();
    // // 1045375
    // eprintln!("borsh size: {}", borsh::to_vec(&data).unwrap().len());
    bencher.iter(|| {
        borsh::to_vec(&data).unwrap()
    });
}


fn serialize_rkyv_asis(bencher: &mut Bencher) {
    let data: InterpreterData = serde_json::from_str(DATA_STR).unwrap();
    bencher.iter(|| {
        let mut ser = rkyv::ser::serializers::AllocSerializer::<4096>::default();
        rkyv::Serialize::serialize(black_box(&data), &mut ser).unwrap();

        ser.into_serializer().into_inner()
    });
}

fn serialize_rkyv_dedup(bencher: &mut Bencher) {
    let data: InterpreterData = serde_json::from_str(DATA_STR).unwrap();
    let data = dedup(data);

    // // 717696
    // {
    //     let mut ser = rkyv::ser::serializers::AllocSerializer::<4096>::default();
    //     rkyv::Serialize::serialize(black_box(&data), &mut ser).unwrap();

    //     let size = ser.into_serializer().into_inner().len();
    //     eprintln!("rkyv_dedup size: {}", size);
    // }
    bencher.iter(|| {
        let mut ser = rkyv::ser::serializers::AllocSerializer::<4096>::default();
        rkyv::Serialize::serialize(black_box(&data), &mut ser).unwrap();

        ser.into_serializer().into_inner()
    });
}

fn dedup(mut particle: InterpreterData) -> InterpreterData {
    use air_interpreter_data::ValueRef::*;
    use air_interpreter_data::*;
    use std::rc::Rc;

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

    particle
}

benchmark_group!(
    benches,
    deserialize_serde_json,
    deserialize_serde_json2,
    deserialize_simd_json_invalid,
    // fails because of serde_json::..::RawValue
    // deserialize_ciborium,

    deserialize_rkyv_dedup_validate_only,
    deserialize_rkyv_dedup,

    serialize_serde_json,
    serialize_simd_json,
    serialize_ciborium,
    serialize_borshium,
    serialize_rkyv_asis,
    serialize_rkyv_dedup,
);

benchmark_main!(benches);
