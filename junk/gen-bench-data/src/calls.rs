use crate::Data;

use air_test_utils::key_utils::derive_dummy_keypair;
use air_test_utils::prelude::*;
use maplit::hashmap;

static AIR_SCRIPT: &str = r#"
(seq
   (call %init_peer_id% ("init" "init") [] data)
   (fold data i
       (par
          (call %init_peer_id% ("handle" "handle") [i i i i i i i i i i i i i i i i i i i i i i i i] $stream)
          (next i))))
"#;

pub(crate) fn call_requests(size: usize) -> Data {
    let values: Vec<_> = (0..size).map(|n| format!("{n}")).collect();
    let data = json!(values);
    let init_peer_name = "peer_id";
    let particle_id = "particle_id";
    let (init_peer_keypair, init_peer_id) = derive_dummy_keypair(init_peer_name);

    let mut avm = create_avm_with_key::<NativeAirRunner>(
        init_peer_keypair.clone(),
        // actually, is not used
        set_variable_call_service(data.clone()),
        <_>::default(),
    );

    let init_call_results: CallResults = <_>::default();

    let res_init = avm
        .call_single(
            AIR_SCRIPT,
            "",
            "",
            init_peer_id.clone(),
            0,
            0,
            None,
            init_call_results,
            particle_id,
        )
        .unwrap();

    // initialization call requests
    let call_results: CallResults = res_init
        .call_requests
        .keys()
        .map(|id| (*id, CallServiceResult::ok(data.clone())))
        .collect();

    Data {
        air: AIR_SCRIPT.to_owned(),
        prev_data: res_init.data,
        cur_data: vec![],
        params_json: hashmap! {
            "comment".to_owned() => "multiple call requests".to_owned(),
            "particle-id".to_owned() => particle_id.to_owned(),
            "current-peer-id".to_owned() => init_peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: Some(call_results),
        keypair: bs58::encode(init_peer_keypair.as_inner().to_vec()).into_string(),
    }
}

pub(crate) fn call_results(size: usize) -> Data {
    let values: Vec<_> = (0..size).map(|n| format!("{n}")).collect();
    let data = json!(values);
    let init_peer_name = "peer_id";
    let particle_id = "particle_id";
    let (init_peer_keypair, init_peer_id) = derive_dummy_keypair(init_peer_name);

    let mut avm = create_avm_with_key::<NativeAirRunner>(
        init_peer_keypair.clone(),
        // actually, is not used
        set_variable_call_service(data.clone()),
        <_>::default(),
    );

    let call_results: CallResults = <_>::default();

    let res_init = avm
        .call_single(
            AIR_SCRIPT,
            "",
            "",
            init_peer_id.clone(),
            0,
            0,
            None,
            call_results,
            particle_id,
        )
        .unwrap();

    // initialization call requests
    let init_call_results: CallResults = res_init
        .call_requests
        .keys()
        .map(|id| (*id, CallServiceResult::ok(data.clone())))
        .collect();

    let res = avm
        .call_single(
            AIR_SCRIPT,
            res_init.data,
            "",
            init_peer_id.clone(),
            0,
            0,
            None,
            init_call_results,
            particle_id,
        )
        .unwrap();

    let call_results: CallResults = res
        .call_requests
        .iter()
        .map(|(id, req)| (*id, CallServiceResult::ok(req.arguments.clone().into())))
        .collect();

    Data {
        air: AIR_SCRIPT.to_owned(),
        prev_data: res.data,
        cur_data: vec![],
        params_json: hashmap! {
            "comment".to_owned() => "multiple call results".to_owned(),
            "particle-id".to_owned() => particle_id.to_owned(),
            "current-peer-id".to_owned() => init_peer_id.clone(),
            "init-peer-id".to_owned() => init_peer_id,
        },
        call_results: Some(call_results),
        keypair: bs58::encode(init_peer_keypair.as_inner().to_vec()).into_string(),
    }
}
