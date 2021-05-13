use air_test_utils::call_vm;
use air_test_utils::create_avm;
use air_test_utils::executed_state;
use air_test_utils::unit_call_service;

#[test]
fn seq_remote_remote() {
    let mut vm = create_avm(unit_call_service(), "");

    let script = String::from(
        r#"
            (seq
                (call "remote_peer_id_1" ("local_service_id" "local_fn_name") [] result_name)
                (call "remote_peer_id_2" ("service_id" "fn_name") [] g)
            )"#,
    );

    let res = call_vm!(vm, "asd", script.clone(), "", "");
    assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id_1")]);

    let initial_data = vec![executed_state::scalar_string("")];
    let initial_data = serde_json::to_string(&initial_data).expect("default serializer shouldn't fail");

    let res = call_vm!(vm, "asd", script, "", initial_data);
    assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id_2")]);
}

#[test]
fn seq_local_remote() {
    let local_peer_id = "local_peer_id";
    let remote_peer_id = String::from("remote_peer_id");
    let mut vm = create_avm(unit_call_service(), local_peer_id);

    let script = format!(
        r#"
            (seq
                (call "{}" ("local_service_id" "local_fn_name") [] result_name)
                (call "{}" ("service_id" "fn_name") [] g)
            )"#,
        local_peer_id, remote_peer_id
    );

    let res = call_vm!(vm, "asd", script, "[]", "[]");
    assert_eq!(res.next_peer_pks, vec![remote_peer_id]);
}
