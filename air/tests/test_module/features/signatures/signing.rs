/*
 * Copyright 2023 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use air_interpreter_signatures::PeerCidTracker;
use air_test_framework::{ephemeral::PeerId, AirScriptExecutor};
use air_test_utils::key_utils::derive_dummy_keypair;
use air_test_utils::prelude::*;
use air_test_utils::test_runner::TestRunParameters;

use futures::stream::StreamExt;

#[tokio::test]
async fn test_signature_empty() {
    let script = "(null)";
    let init_peer_name = "init_peer_id";
    let (keypair, _) = derive_dummy_keypair(init_peer_name);

    let exec = <AirScriptExecutor>::new(
        TestRunParameters::from_init_peer_id(init_peer_name),
        vec![],
        vec![PeerId::from(init_peer_name)].into_iter(),
        script,
    )
        .await
    .unwrap();
    let res = exec.execute_one(init_peer_name).await.unwrap();
    assert_eq!(res.ret_code, 0, "{:?}", res);

    let data = borsh::to_vec(&(vec![""; 0], "")).unwrap();
    let expected_signature: air_interpreter_signatures::Signature = keypair.sign(&data).unwrap().into();

    let data = data_from_result(&res);
    let signature = data.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", data.signatures);
}

#[tokio::test]
async fn test_signature_call_var() {
    let init_peer_name = "init_peer_id";
    let (keypair, init_peer_id) = derive_dummy_keypair(init_peer_name);

    let air_script = format!(
        r#"
        (call "{init_peer_name}" ("" "") [] var) ; ok = "ok"
        "#
    );
    let exec =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &air_script).await.unwrap();


    let exec_results = exec.execution_iter(init_peer_id.as_str()).unwrap().collect::<Vec<_>>().await;
    let res = exec_results.last().unwrap();
    assert_eq!(res.ret_code, 0, "{:?}", res);
    let data = data_from_result(&res);

    let expected_call_state = scalar!("ok", peer = &init_peer_id, service = "..0");
    let expected_cid = extract_service_result_cid(&expected_call_state);

    let mut expected_tracker = PeerCidTracker::new(init_peer_id.clone());
    expected_tracker.register(&init_peer_id, &expected_cid);
    let expected_signature = expected_tracker.gen_signature("", &keypair).unwrap();

    let signature = data.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", data.signatures);
}

#[tokio::test]
async fn test_signature_call_stream() {
    let init_peer_name = "init_peer_id";
    let air_script = format!(
        r#"
        (call "{init_peer_name}" ("" "") [] $var) ; ok = "ok"
        "#
    );
    let exec =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &air_script).await.unwrap();

    let exec_results = exec.execution_iter(init_peer_name).unwrap().collect::<Vec<_>>().await;
    let res = exec_results.last().unwrap();
    assert_eq!(res.ret_code, 0, "{:?}", res);
    let data = data_from_result(&res);

    let (keypair, init_peer_id) = derive_dummy_keypair(init_peer_name);

    let expected_call_state = stream!("ok", 0, peer = &init_peer_id, service = "..0");
    let expected_cid = extract_service_result_cid(&expected_call_state);

    let mut expected_tracker = PeerCidTracker::new(init_peer_id.clone());
    expected_tracker.register(&init_peer_id, &expected_cid);
    let expected_signature = expected_tracker.gen_signature("", &keypair).unwrap();

    let signature = data.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", data.signatures);
}

#[tokio::test]
async fn test_signature_call_unused() {
    let init_peer_name = "init_peer_id";
    let air_script = format!(
        r#"
        (call "{init_peer_name}" ("" "") []) ; ok = "ok"
        "#
    );
    let exec =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &air_script).await.unwrap();

    let exec_results = exec.execution_iter(init_peer_name).unwrap().collect::<Vec<_>>().await;
    let res = exec_results.last().unwrap();
    assert_eq!(res.ret_code, 0, "{:?}", res);
    let data = data_from_result(&res);

    let (keypair, init_peer_id) = derive_dummy_keypair(init_peer_name);

    let expected_tracker = PeerCidTracker::new(init_peer_id.to_owned());
    let expected_signature = expected_tracker.gen_signature("", &keypair).unwrap();

    let signature = data.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", data.signatures);
}

#[tokio::test]
async fn test_signature_call_merged() {
    let init_peer_name = "init_peer_id";
    let other_peer_name = "other_peer_id";

    let air_script = format!(
        r#"
    (seq
       (call "{init_peer_name}" ("" "") [] x) ; ok = "res0"
       (seq
          (call "{other_peer_name}" ("" "") [] y) ; ok = "res1"
          (call "{init_peer_name}" ("" "") [] z) ; ok = "res2"
       ))
    "#
    );

    let exec =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &air_script).await.unwrap();
    let _ = exec.execute_one(init_peer_name).await.unwrap();
    let _ = exec.execute_one(other_peer_name).await.unwrap();
    let res2 = exec.execute_one(init_peer_name).await.unwrap();
    let data2 = data_from_result(&res2);

    let expected_call_state0 = scalar!("res0", peer_name = init_peer_name, service = "..0");
    let expected_cid0 = extract_service_result_cid(&expected_call_state0);
    let expected_call_state2 = scalar!("res2", peer_name = init_peer_name, service = "..2");
    let expected_cid2 = extract_service_result_cid(&expected_call_state2);

    let (keypair, _) = derive_dummy_keypair(init_peer_name);

    let mut expected_tracker = PeerCidTracker::new(init_peer_name.to_owned());
    expected_tracker.register(init_peer_name, &expected_cid0);
    expected_tracker.register(init_peer_name, &expected_cid2);
    let expected_signature = expected_tracker.gen_signature("", &keypair).unwrap();

    let signature = data2.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", data2.signatures);
}

#[tokio::test]
async fn test_signature_call_twice() {
    // Test that if some CID appears twice in the call result, it is accounted twice.
    let init_peer_name = "init_peer_id";

    let (keypair, init_peer_id) = derive_dummy_keypair(init_peer_name);

    let air_script = format!(
        r#"
        (seq
            (seq (ap 1 $s) (ap 2 $s))
            (fold $s i
                (seq
                    (call "{init_peer_name}" ("" "") [] var) ; ok = "ok"
                    (next i))))
        "#
    );
    let exec =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &air_script).await.unwrap();

    let exec_results = exec.execution_iter(init_peer_id.as_str()).unwrap().collect::<Vec<_>>().await;
    let res = exec_results.last().unwrap();
    assert_eq!(res.ret_code, 0, "{:?}", res);
    let data = data_from_result(&res);

    let expected_call_state = scalar!("ok", peer = &init_peer_id, service = "..0");
    let expected_cid = extract_service_result_cid(&expected_call_state);

    let mut unexpected_tracker = PeerCidTracker::new(init_peer_id.to_owned());
    unexpected_tracker.register(&init_peer_id, &expected_cid);
    let unexpected_signature = unexpected_tracker.gen_signature("", &keypair).unwrap();

    let mut expected_tracker = PeerCidTracker::new(init_peer_id.to_owned());
    expected_tracker.register(&init_peer_id, &expected_cid);
    expected_tracker.register(&init_peer_id, &expected_cid);
    let expected_signature = expected_tracker.gen_signature("", &keypair).unwrap();

    assert_ne!(expected_signature, unexpected_signature, "test is incorrect");

    let signature = data.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", data.signatures);
}

#[tokio::test]
async fn test_signature_canon_basic() {
    let init_peer_name = "init_peer_id";
    let (keypair, init_peer_id) = derive_dummy_keypair(init_peer_name);

    let air_script = format!(
        r#"
       (seq
          (call "{init_peer_name}" ("serv" "func") [] items) ; ok = [1, 2, 3]
          (seq
             (fold items i
                (seq
                   (ap i $stream)
                   (next i)))
             (canon "{init_peer_name}" $stream #canon)))
    "#
    );
    let exec =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &air_script).await.unwrap();

    let exec_results = exec.execution_iter(init_peer_name).unwrap().collect::<Vec<_>>().await;
    let res = exec_results.last().unwrap();
    let last_data = data_from_result(&res);

    let expected_call_result = scalar!(
        json!([1, 2, 3]),
        peer = &init_peer_id,
        service = "serv..0",
        function = "func"
    );
    let expected_call_result_cid = extract_service_result_cid(&expected_call_result);

    let expected_canon_state = canon(json!({
        "tetraplet": {"peer_pk": init_peer_id, "service_id": "", "function_name": "", "json_path": ""},
        "values": [{
            "result": 1,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "serv..0",
                "function_name": "func",
                "json_path": ".$.[0]",
            },
            "provenance": Provenance::service_result(expected_call_result_cid.clone()),
        }, {
            "result": 2,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "serv..0",
                "function_name": "func",
                "json_path": ".$.[1]",
            },
            "provenance": Provenance::service_result(expected_call_result_cid.clone()),
        }, {
            "result": 3,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "serv..0",
                "function_name": "func",
                "json_path": ".$.[2]",
            },
            "provenance": Provenance::service_result(expected_call_result_cid.clone()),
        }]
    }));
    let expected_canon_cid = extract_canon_result_cid(&expected_canon_state);

    let mut expected_tracker = PeerCidTracker::new(init_peer_name.to_owned());
    expected_tracker.register(init_peer_name, &expected_canon_cid);
    expected_tracker.register(init_peer_name, &expected_call_result_cid);
    let expected_signature = expected_tracker.gen_signature("", &keypair).unwrap();

    let signature = last_data.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", last_data);
}

#[tokio::test]
async fn test_signature_canon_merge() {
    let init_peer_name = "init_peer_id";
    let other_peer_name = "other_peer_id";
    let (keypair, init_peer_id) = derive_dummy_keypair(init_peer_name);

    let air_script = format!(
        r#"
        (seq
           (seq
              (call "{init_peer_name}" ("serv" "func") [] items) ; ok = [1, 2, 3]
              (seq
                 (fold items i
                    (seq
                       (ap i $stream)
                       (next i)))
                 (canon "{init_peer_name}" $stream #canon)))
           (seq
              (call "{other_peer_name}" ("" "") []) ; ok = "ok"
              (call "{init_peer_name}" ("" "") []))) ; ok = "ok"
    "#
    );
    let exec =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &air_script).await.unwrap();

    exec.execute_all(init_peer_name).await;
    exec.execute_one(other_peer_name).await;

    let exec_results = exec.execution_iter(init_peer_name).unwrap().collect::<Vec<_>>().await;
    let res = exec_results.last().unwrap();
    let last_data = data_from_result(&res);

    let expected_call_result = scalar!(
        json!([1, 2, 3]),
        peer = &init_peer_id,
        service = "serv..0",
        function = "func"
    );
    let expected_call_result_cid = extract_service_result_cid(&expected_call_result);

    let expected_canon_state = canon(json!({
        "tetraplet": {"peer_pk": init_peer_id, "service_id": "", "function_name": "", "json_path": ""},
        "values": [{
            "result": 1,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "serv..0",
                "function_name": "func",
                "json_path": ".$.[0]",
            },
            "provenance": Provenance::service_result(expected_call_result_cid.clone()),
        }, {
            "result": 2,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "serv..0",
                "function_name": "func",
                "json_path": ".$.[1]",
            },
            "provenance": Provenance::service_result(expected_call_result_cid.clone()),
        }, {
            "result": 3,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "serv..0",
                "function_name": "func",
                "json_path": ".$.[2]",
            },
            "provenance": Provenance::service_result(expected_call_result_cid.clone()),
        }]
    }));
    let expected_canon_cid = extract_canon_result_cid(&expected_canon_state);

    let mut expected_tracker = PeerCidTracker::new(init_peer_name.to_owned());
    expected_tracker.register(init_peer_name, &expected_canon_cid);
    expected_tracker.register(init_peer_name, &expected_call_result_cid);
    let expected_signature = expected_tracker.gen_signature("", &keypair).unwrap();

    let signature = last_data.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", last_data);
}

#[tokio::test]
async fn test_signature_canon_result() {
    // this test checks that call result in canon doesn't lead to repeadted accounting of the call result
    let init_peer_name = "init_peer_id";
    let (keypair, init_peer_id) = derive_dummy_keypair(init_peer_name);

    let air_script = format!(
        r#"
        (seq
           (seq
              (call "{init_peer_name}" ("serv" "func") [] items) ; ok = [1, 2, 3]
              (fold items i
                 (seq
                    (ap i $stream)
                    (next i))))
           (seq
              (call "{init_peer_name}" ("serv" "func2") [] $stream) ; ok = 42
              (canon "{init_peer_name}" $stream #canon)))
    "#
    );
    let exec =
        AirScriptExecutor::from_annotated(TestRunParameters::from_init_peer_id(init_peer_name), &air_script).await.unwrap();

    let exec_results = exec.execution_iter(init_peer_name).unwrap().collect::<Vec<_>>().await;
    let res = exec_results.last().unwrap();
    let last_data = data_from_result(&res);

    let expected_call_result1 = scalar!(
        json!([1, 2, 3]),
        peer = &init_peer_id,
        service = "serv..0",
        function = "func"
    );
    let expected_call_result_cid1 = extract_service_result_cid(&expected_call_result1);

    let expected_call_result2 = stream!(
        json!(42),
        1,
        peer = &init_peer_id,
        service = "serv..1",
        function = "func2"
    );
    let expected_call_result_cid2 = extract_service_result_cid(&expected_call_result2);

    let expected_canon_state = canon(json!({
        "tetraplet": {"peer_pk": init_peer_id, "service_id": "", "function_name": "", "json_path": ""},
        "values": [{
            "result": 1,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "serv..0",
                "function_name": "func",
                "json_path": ".$.[0]",
            },
            "provenance": Provenance::service_result(expected_call_result_cid1.clone()),
        }, {
            "result": 2,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "serv..0",
                "function_name": "func",
                "json_path": ".$.[1]",
            },
            "provenance": Provenance::service_result(expected_call_result_cid1.clone()),
        }, {
            "result": 3,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "serv..0",
                "function_name": "func",
                "json_path": ".$.[2]",
            },
            "provenance": Provenance::service_result(expected_call_result_cid1.clone()),
        }, {
            "result": 42,
            "tetraplet": {
                "peer_pk": init_peer_id,
                "service_id": "serv..1",
                "function_name": "func2",
                "json_path": "",
            },
            "provenance": Provenance::service_result(expected_call_result_cid2.clone()),
        }]
    }));
    let expected_canon_cid = extract_canon_result_cid(&expected_canon_state);

    let mut expected_tracker = PeerCidTracker::new(init_peer_name.to_owned());
    expected_tracker.register(init_peer_name, &expected_call_result_cid1);
    expected_tracker.register(init_peer_name, &expected_call_result_cid2);
    expected_tracker.register(init_peer_name, &expected_canon_cid);
    let expected_signature = expected_tracker.gen_signature("", &keypair).unwrap();

    let signature = last_data.signatures.get(&keypair.public().into());
    assert_eq!(signature, Some(&expected_signature), "{:?}", last_data);
}
