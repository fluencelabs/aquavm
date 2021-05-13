use air_test_utils::call_vm;
use air_test_utils::create_avm;
use air_test_utils::unit_call_service;

#[test]
fn par_remote_remote() {
    use std::collections::HashSet;

    let mut vm = create_avm(unit_call_service(), "");

    let script = String::from(
        r#"
            (par
                (call "remote_peer_id_1" ("local_service_id" "local_fn_name") [] result_name)
                (call "remote_peer_id_2" ("service_id" "fn_name") [] g)
            )"#,
    );

    let mut res = call_vm!(vm, "", script, "[]", "[]");

    let peers_result: HashSet<_> = res.next_peer_pks.drain(..).collect();
    let peers_right: HashSet<_> = maplit::hashset!(String::from("remote_peer_id_1"), String::from("remote_peer_id_2"));

    assert_eq!(peers_result, peers_right);
}

#[test]
fn par_local_remote() {
    let local_peer_id = "local_peer_id";
    let mut vm = create_avm(unit_call_service(), local_peer_id);

    let script = format!(
        r#"
            (par
                (call "{}" ("local_service_id" "local_fn_name") [] result_name)
                (call "remote_peer_id_2" ("service_id" "fn_name") [] g)
            )"#,
        local_peer_id
    );

    let res = call_vm!(vm, "", script, "[]", "[]");

    assert_eq!(res.next_peer_pks, vec![String::from("remote_peer_id_2")]);
}
