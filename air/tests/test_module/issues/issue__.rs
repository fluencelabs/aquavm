/*
 * Copyright 2022 Fluence Labs Limited
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

use air_test_utils::prelude::*;

static PREV_DATA: &'static str = r#"{"version":"0.6.2","interpreter_version":"0.35.1","trace":[{"call":{"executed":{"scalar":"bagaaiera5vlhrcij57bv65swyy2n7aoxaec44q7n3mimmznc7ukttguhu62q"}}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq","values":["bagaaieramioiqtlgc6g334wxxg3j2cwnynrcrlj7nuegbzv5agay7hi4qvga","bagaaieraf765opx5uxfpppttpweakkm5qewp5icxktxpc7yxawjbgto55bfa","bagaaieravzgdpow36er3ujnffac6bmiahyst53ge4g5xltmishrs4h3q2rma","bagaaieranwnbnp7gel327afdpejz7cfvukblotyzpr2fnhhqg32jqlxw6ydq","bagaaieranr77szmrnypllnnartxh65522fwghaeh26t3ifvk6hzxwr6jgsyq","bagaaieraqz4ptlijr3yv2givsljisbaurlbmc2hpjklemidefxcexy5qc76q"]}},{"par":[1,0]},{"call":{"sent_by":"12D3KooWHwkQZw95tZwmj6r5LBrbazko9mynDoXyKdeKvcGEV1bQ: 2"}},{"par":[21,0]},{"par":[2,18]},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"sent_by":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU"}},{"par":[5,12]},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"executed":{"scalar":"bagaaierajovfl66ofc7wbiyzd4pqbkxfxbcg6vi6bxucona2s45lsaznanzq"}}},{"call":{"executed":{"scalar":"bagaaierahwyub3r2hat5po4l2m33vxiwesijonyekwayfrmh7pjf6fuactkq"}}},{"call":{"executed":{"scalar":"bagaaieratydazpgclzatyacesaea7xy6jyirabmz2xvysroyoaffqfwn2ziq"}}},{"call":{"sent_by":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU"}},{"par":[2,9]},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"sent_by":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU"}},{"par":[2,6]},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"sent_by":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU"}},{"par":[2,3]},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"sent_by":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU"}},{"par":[2,0]},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"sent_by":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU"}},{"call":{"sent_by":"12D3KooWHwkQZw95tZwmj6r5LBrbazko9mynDoXyKdeKvcGEV1bQ: 3"}}],"streams":{},"r_streams":{"$services":{"1108":[0,0,0,0,0,0]},"$array-inline":{"115":[1]},"$dummy_test":{"8897":[0]},"$dummy":{"884":[0]},"$aliases":{"1636":[0]}},"lcid":9,"cid_info":{"value_store":{"bagaaieravto4z7jk4evzpw5uwfjttqweeyg5upno6oxdil5roe3f5pv53jeq":"12D3KooWMMGdfVEJ1rWe1nH1nehYDzNEHhg5ogdfiGk88AupCMnf","bagaaierav7be43nre2jnzsixzxwr3f5p2lfyjphqeliety5pwzac64hjep4a":"12D3KooWAKNos2KogexTXhrkMZzFYpLHuWJ4PgoAhurSAv7o5CWA","bagaaierajovfl66ofc7wbiyzd4pqbkxfxbcg6vi6bxucona2s45lsaznanzq":{"error":"","strings":["{\"deal_id\":\"494544a31723001048e1a06bdb5e50f70c8410aa\",\"spell_id\":\"f8332c1c-e22f-4dd5-bf67-b96cb2cacd8b\",\"worker_id\":\"12D3KooWHGf75aqtNgynPpyrKdL7HyfsYThd9kM6fY1rtnZwgfAx\"}","{\"deal_id\":\"bca50bbd702cf3f22f14c48f221141000563e1d7\",\"spell_id\":\"143460f2-eb2a-4de4-bdc9-2fba279f37da\",\"worker_id\":\"12D3KooWQYXeECgpXZKSpURhxd6bKVaGZ89VwMBERrKRrX5uGu8U\"}","{\"deal_id\":\"2ed56e62b96fed6da7455351492435ca56b2e555\",\"spell_id\":\"4191d648-1113-4600-adc4-52bd94c237a6\",\"worker_id\":\"12D3KooWAQ5YgVhECtWjrdy9MncomrgAWy8yHgHhc5bXGEeEjSnQ\"}","{\"deal_id\":\"173fbe9e6afe7abb2bc4de8596fede7a69b02ba0\",\"spell_id\":\"be9940cf-d1af-41ee-8be8-6511b0884f63\",\"worker_id\":\"12D3KooWCyNS5VaYJghFEgFySjKUNGhsSjyztkvcV7hz3BMF7Q6W\"}","{\"deal_id\":\"2f501adeef0a9f06f63794ce499161ec8f49c449\",\"spell_id\":\"0a6672bc-3667-4509-9265-37d45905d3a4\",\"worker_id\":\"12D3KooWRUT6mz6yBSzSmS69jpoABWmiJWRrQUvTE2xw3uVgZguQ\"}"],"success":true},"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta":"","bagaaierahfrmfzj2xvwbiw4oavwpjckj2qfd6qabvs3df6ykotpomtpkfx4a":"12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv","bagaaiera5vlhrcij57bv65swyy2n7aoxaec44q7n3mimmznc7ukttguhu62q":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU","bagaaieraqbmy7wg4mvfmll4uqjmkucsk4a6ychwrgyrdji6uicfrav7jsqya":"12D3KooWDcpWuyrMTDinqNgmXAuRdfd2mTdY9VoXZSAet2pDzh6r","bagaaierahwyub3r2hat5po4l2m33vxiwesijonyekwayfrmh7pjf6fuactkq":{"deal_id":"494544a31723001048e1a06bdb5e50f70c8410aa","spell_id":"f8332c1c-e22f-4dd5-bf67-b96cb2cacd8b","worker_id":"12D3KooWHGf75aqtNgynPpyrKdL7HyfsYThd9kM6fY1rtnZwgfAx"},"bagaaieratydazpgclzatyacesaea7xy6jyirabmz2xvysroyoaffqfwn2ziq":{"aliases":["worker-spell"],"blueprint_id":"e81917bfb96579dc53531df5a6e490e03702991ccf51d66cd511dd124354af7c","id":"f8332c1c-e22f-4dd5-bf67-b96cb2cacd8b","owner_id":"12D3KooWHGf75aqtNgynPpyrKdL7HyfsYThd9kM6fY1rtnZwgfAx","worker_id":"12D3KooWHGf75aqtNgynPpyrKdL7HyfsYThd9kM6fY1rtnZwgfAx"},"bagaaieraqmk656lhlznfqp7qt36dkgpbl5zb6mdr2jfrlibuf7jtnwfy5daa":"12D3KooWJ4bTHirdTFNZpCS72TAzwtdmavTBkkEXtzo6wHL25CtE"},"tetraplet_store":{"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq":{"peer_pk":"12D3KooWHwkQZw95tZwmj6r5LBrbazko9mynDoXyKdeKvcGEV1bQ","service_id":"","function_name":"","json_path":""}},"canon_store":{"bagaaieramioiqtlgc6g334wxxg3j2cwnynrcrlj7nuegbzv5agay7hi4qvga":{"value":"bagaaierahfrmfzj2xvwbiw4oavwpjckj2qfd6qabvs3df6ykotpomtpkfx4a","tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq"},"bagaaieravzgdpow36er3ujnffac6bmiahyst53ge4g5xltmishrs4h3q2rma":{"value":"bagaaieravto4z7jk4evzpw5uwfjttqweeyg5upno6oxdil5roe3f5pv53jeq","tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq"},"bagaaieranwnbnp7gel327afdpejz7cfvukblotyzpr2fnhhqg32jqlxw6ydq":{"value":"bagaaieraqmk656lhlznfqp7qt36dkgpbl5zb6mdr2jfrlibuf7jtnwfy5daa","tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq"},"bagaaieraf765opx5uxfpppttpweakkm5qewp5icxktxpc7yxawjbgto55bfa":{"value":"bagaaiera5vlhrcij57bv65swyy2n7aoxaec44q7n3mimmznc7ukttguhu62q","tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq"},"bagaaieranr77szmrnypllnnartxh65522fwghaeh26t3ifvk6hzxwr6jgsyq":{"value":"bagaaierav7be43nre2jnzsixzxwr3f5p2lfyjphqeliety5pwzac64hjep4a","tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq"},"bagaaieraqz4ptlijr3yv2givsljisbaurlbmc2hpjklemidefxcexy5qc76q":{"value":"bagaaieraqbmy7wg4mvfmll4uqjmkucsk4a6ychwrgyrdji6uicfrav7jsqya","tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq"}}}}"#;
static CURRENT_DATA: &'static str = r#"{"version":"0.6.2","interpreter_version":"0.35.1","trace":[{"call":{"executed":{"scalar":"bagaaiera5vlhrcij57bv65swyy2n7aoxaec44q7n3mimmznc7ukttguhu62q"}}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq","values":["bagaaieramioiqtlgc6g334wxxg3j2cwnynrcrlj7nuegbzv5agay7hi4qvga","bagaaieraf765opx5uxfpppttpweakkm5qewp5icxktxpc7yxawjbgto55bfa","bagaaieravzgdpow36er3ujnffac6bmiahyst53ge4g5xltmishrs4h3q2rma","bagaaieranwnbnp7gel327afdpejz7cfvukblotyzpr2fnhhqg32jqlxw6ydq","bagaaieranr77szmrnypllnnartxh65522fwghaeh26t3ifvk6hzxwr6jgsyq","bagaaieraqz4ptlijr3yv2givsljisbaurlbmc2hpjklemidefxcexy5qc76q"]}},{"par":[1,0]},{"call":{"sent_by":"12D3KooWHwkQZw95tZwmj6r5LBrbazko9mynDoXyKdeKvcGEV1bQ: 2"}},{"par":[37,0]},{"par":[2,34]},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"sent_by":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU"}},{"par":[2,31]},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"sent_by":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU: 7"}},{"par":[21,9]},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"executed":{"scalar":"bagaaieraq4d7phvb22lxqn2hah6qdlrmnd4e2bnl7lv6xtctsf4om3t7he7q"}}},{"call":{"executed":{"scalar":"bagaaieraenwuf6krr4pdf4hfmbji2os5tzpegkdm6lsvzenbtxhjtbkydiza"}}},{"call":{"executed":{"scalar":"bagaaieracv5nvv5m243s2liqazvhtxxu5mue6vtyy55cov5c2cb4z7rzseqq"}}},{"call":{"executed":{"scalar":"bagaaieray4dv2yxl3dv4qijj43wv4nrogxygiqzbm4mai6atxplerr6cgsqa"}}},{"ap":{"gens":[0]}},{"call":{"executed":{"scalar":"bagaaieraww7kig3mmi7xycprx4snzlsy5ovtydg5scwzm26ehjc3isdh4evq"}}},{"ap":{"gens":[0]}},{"call":{"executed":{"scalar":"bagaaieraww7kig3mmi7xycprx4snzlsy5ovtydg5scwzm26ehjc3isdh4evq"}}},{"ap":{"gens":[1]}},{"par":[2,1]},{"canon":{"tetraplet":"bagaaierat3u264fleh5ijseosepshhf6rjtuxqpse3xwdb4qubwxmhuyykka","values":["bagaaierazufhm2aabmdz4cxsqhbsjd2quonhfr37pg3wgegjryqovcxjygnq"]}},{"call":{"executed":{"scalar":"bagaaierahekvfqezyea3cmp6v4smk6k2nik3zdwieakuetqnfnbhji3juc7q"}}},{"call":{"executed":{"scalar":"bagaaiera2efexspayh5e5dz5ptrfck4hk3shzjp2iuptopbzufbrxoenwspq"}}},{"call":{"executed":{"scalar":"bagaaierainu3q2p6dtfmjdb335pkptdc3ejy7ks3ikjk5ep5xmx7pi7cxd6q"}}},{"par":[2,0]},{"canon":{"tetraplet":"bagaaierat3u264fleh5ijseosepshhf6rjtuxqpse3xwdb4qubwxmhuyykka","values":["bagaaierazyao5vrb6ymiwr5qufov4y66gajm5t5g64a4ealekzleyforlvcq","bagaaiera25nqqxd2e3nqhchhvqyolu5ezwwysrrvzl7wdnplyojvw7swmv2a"]}},{"call":{"executed":{"scalar":"bagaaierari6kmcj5ho6vmeolypki3tq2oof2ycvwb45fytbvx5lvhlcsfegq"}}},{"call":{"executed":{"scalar":"bagaaierareylqppva426wdkls4xyyfc4svxi7r4rdcl6ece7mttgeiyj3mva"}}},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"sent_by":"12D3KooWMMGdfVEJ1rWe1nH1nehYDzNEHhg5ogdfiGk88AupCMnf"}},{"par":[2,6]},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"sent_by":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU"}},{"par":[2,3]},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"sent_by":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU"}},{"par":[2,0]},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"sent_by":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU"}},{"call":{"sent_by":"12D3KooWHwkQZw95tZwmj6r5LBrbazko9mynDoXyKdeKvcGEV1bQ: 3"}}],"streams":{},"r_streams":{"$dummy":{"884":[0]},"$dummy_test":{"8897":[0]},"$array-inline":{"115":[1]},"$services":{"1108":[0,0,1,0,0,0]},"$aliases":{"1636":[2]}},"lcid":4,"cid_info":{"value_store":{"bagaaiera3lvpinpzeqo3ebmr3mwhpqds72bbv4naxxuvjczkku2jhdg74uaq":"worker-spell","bagaaieraq4d7phvb22lxqn2hah6qdlrmnd4e2bnl7lv6xtctsf4om3t7he7q":{"error":"","strings":["{\"deal_id\":\"494544a31723001048e1a06bdb5e50f70c8410aa\",\"spell_id\":\"3c9b002b-e6f3-40ef-a347-b4ea49fdc0f2\",\"worker_id\":\"12D3KooWLQNJT82ck4CUVuccyvtVE42SnadCg8mLF28Jc4fpCNHK\"}","{\"deal_id\":\"bca50bbd702cf3f22f14c48f221141000563e1d7\",\"spell_id\":\"5da1ef6f-694b-412e-be69-98910810ac75\",\"worker_id\":\"12D3KooWCwkdR1B4Umum9CSFwswv1PLwxuJxcbNcRi2iwzcxeZ6E\"}","{\"deal_id\":\"2ed56e62b96fed6da7455351492435ca56b2e555\",\"spell_id\":\"0936e15a-e847-4cd3-a81e-c7fc477630b4\",\"worker_id\":\"12D3KooWQHQv1RKc1rrhbJKAArRsnT9A1sdmR4jssxyF2J2WaQpk\"}","{\"deal_id\":\"173fbe9e6afe7abb2bc4de8596fede7a69b02ba0\",\"spell_id\":\"df2d993e-7351-41b3-ab17-ccc0346f3486\",\"worker_id\":\"12D3KooWKdkVJM1Utd3FmzaWo5t7a2YVhHh1mCYLn8ozv6mEabZv\"}","{\"deal_id\":\"2f501adeef0a9f06f63794ce499161ec8f49c449\",\"spell_id\":\"11010476-6224-400f-ae4c-81202a64a425\",\"worker_id\":\"12D3KooWCEdpU2EyfC5VBQN8a6usEwFPZuZTpeCWoYZPiRResSnq\"}"],"success":true},"bagaaieravto4z7jk4evzpw5uwfjttqweeyg5upno6oxdil5roe3f5pv53jeq":"12D3KooWMMGdfVEJ1rWe1nH1nehYDzNEHhg5ogdfiGk88AupCMnf","bagaaierahfrmfzj2xvwbiw4oavwpjckj2qfd6qabvs3df6ykotpomtpkfx4a":"12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv","bagaaiera2efexspayh5e5dz5ptrfck4hk3shzjp2iuptopbzufbrxoenwspq":"5","bagaaierav7be43nre2jnzsixzxwr3f5p2lfyjphqeliety5pwzac64hjep4a":"12D3KooWAKNos2KogexTXhrkMZzFYpLHuWJ4PgoAhurSAv7o5CWA","bagaaiera5vlhrcij57bv65swyy2n7aoxaec44q7n3mimmznc7ukttguhu62q":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU","bagaaieraww7kig3mmi7xycprx4snzlsy5ovtydg5scwzm26ehjc3isdh4evq":true,"bagaaierari6kmcj5ho6vmeolypki3tq2oof2ycvwb45fytbvx5lvhlcsfegq":"[\"worker-spell\",\"adder\"]","bagaaieray4dv2yxl3dv4qijj43wv4nrogxygiqzbm4mai6atxplerr6cgsqa":[{"aliases":["worker-spell"],"blueprint_id":"e81917bfb96579dc53531df5a6e490e03702991ccf51d66cd511dd124354af7c","id":"3c9b002b-e6f3-40ef-a347-b4ea49fdc0f2","owner_id":"12D3KooWLQNJT82ck4CUVuccyvtVE42SnadCg8mLF28Jc4fpCNHK","worker_id":"12D3KooWLQNJT82ck4CUVuccyvtVE42SnadCg8mLF28Jc4fpCNHK"},{"aliases":["adder"],"blueprint_id":"02bc7e80a63963bcd8bfc8b9a3dcb08907d1d72f4a4b3fe07695cb9d32aa1b87","id":"a7942a3f-dd9b-4d7b-acdf-9e895ecbf271","owner_id":"12D3KooWLQNJT82ck4CUVuccyvtVE42SnadCg8mLF28Jc4fpCNHK","worker_id":"12D3KooWLQNJT82ck4CUVuccyvtVE42SnadCg8mLF28Jc4fpCNHK"}],"bagaaierainu3q2p6dtfmjdb335pkptdc3ejy7ks3ikjk5ep5xmx7pi7cxd6q":"1/5","bagaaierahbensrh75a4k6yd5bsn2wephkblsflcrwns64grewr3bn5r4kwkq":"adder","bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta":"","bagaaieraqbmy7wg4mvfmll4uqjmkucsk4a6ychwrgyrdji6uicfrav7jsqya":"12D3KooWDcpWuyrMTDinqNgmXAuRdfd2mTdY9VoXZSAet2pDzh6r","bagaaieraenwuf6krr4pdf4hfmbji2os5tzpegkdm6lsvzenbtxhjtbkydiza":{"deal_id":"494544a31723001048e1a06bdb5e50f70c8410aa","spell_id":"3c9b002b-e6f3-40ef-a347-b4ea49fdc0f2","worker_id":"12D3KooWLQNJT82ck4CUVuccyvtVE42SnadCg8mLF28Jc4fpCNHK"},"bagaaierareylqppva426wdkls4xyyfc4svxi7r4rdcl6ece7mttgeiyj3mva":"12D3KooWMMGdfVEJ1rWe1nH1nehYDzNEHhg5ogdfiGk88AupCMnf 1/5 @ 494544a31723001048e1a06bdb5e50f70c8410aa: [\"worker-spell\",\"adder\"]","bagaaieracv5nvv5m243s2liqazvhtxxu5mue6vtyy55cov5c2cb4z7rzseqq":{"aliases":["worker-spell"],"blueprint_id":"e81917bfb96579dc53531df5a6e490e03702991ccf51d66cd511dd124354af7c","id":"3c9b002b-e6f3-40ef-a347-b4ea49fdc0f2","owner_id":"12D3KooWLQNJT82ck4CUVuccyvtVE42SnadCg8mLF28Jc4fpCNHK","worker_id":"12D3KooWLQNJT82ck4CUVuccyvtVE42SnadCg8mLF28Jc4fpCNHK"},"bagaaieraqmk656lhlznfqp7qt36dkgpbl5zb6mdr2jfrlibuf7jtnwfy5daa":"12D3KooWJ4bTHirdTFNZpCS72TAzwtdmavTBkkEXtzo6wHL25CtE","bagaaierahekvfqezyea3cmp6v4smk6k2nik3zdwieakuetqnfnbhji3juc7q":"1"},"tetraplet_store":{"bagaaierat3u264fleh5ijseosepshhf6rjtuxqpse3xwdb4qubwxmhuyykka":{"peer_pk":"12D3KooWLQNJT82ck4CUVuccyvtVE42SnadCg8mLF28Jc4fpCNHK","service_id":"","function_name":"","json_path":""},"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq":{"peer_pk":"12D3KooWHwkQZw95tZwmj6r5LBrbazko9mynDoXyKdeKvcGEV1bQ","service_id":"","function_name":"","json_path":""},"bagaaierax2o7eimyin4soeqy5n7sws2htwkkhvdmrfcx75mfof5ct32cvqua":{"peer_pk":"12D3KooWLQNJT82ck4CUVuccyvtVE42SnadCg8mLF28Jc4fpCNHK","service_id":"srv","function_name":"list","json_path":".$.aliases.[0]"},"bagaaieraqyuejoftzi7qt5ouy4yvm4euor4yakpahalzyus4fwyy4kpl7ooa":{"peer_pk":"12D3KooWLQNJT82ck4CUVuccyvtVE42SnadCg8mLF28Jc4fpCNHK","service_id":"srv","function_name":"list","json_path":""}},"canon_store":{"bagaaierazufhm2aabmdz4cxsqhbsjd2quonhfr37pg3wgegjryqovcxjygnq":{"value":"bagaaieray4dv2yxl3dv4qijj43wv4nrogxygiqzbm4mai6atxplerr6cgsqa","tetraplet":"bagaaieraqyuejoftzi7qt5ouy4yvm4euor4yakpahalzyus4fwyy4kpl7ooa"},"bagaaierazyao5vrb6ymiwr5qufov4y66gajm5t5g64a4ealekzleyforlvcq":{"value":"bagaaiera3lvpinpzeqo3ebmr3mwhpqds72bbv4naxxuvjczkku2jhdg74uaq","tetraplet":"bagaaierax2o7eimyin4soeqy5n7sws2htwkkhvdmrfcx75mfof5ct32cvqua"},"bagaaieranr77szmrnypllnnartxh65522fwghaeh26t3ifvk6hzxwr6jgsyq":{"value":"bagaaierav7be43nre2jnzsixzxwr3f5p2lfyjphqeliety5pwzac64hjep4a","tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq"},"bagaaieranwnbnp7gel327afdpejz7cfvukblotyzpr2fnhhqg32jqlxw6ydq":{"value":"bagaaieraqmk656lhlznfqp7qt36dkgpbl5zb6mdr2jfrlibuf7jtnwfy5daa","tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq"},"bagaaieramioiqtlgc6g334wxxg3j2cwnynrcrlj7nuegbzv5agay7hi4qvga":{"value":"bagaaierahfrmfzj2xvwbiw4oavwpjckj2qfd6qabvs3df6ykotpomtpkfx4a","tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq"},"bagaaieravzgdpow36er3ujnffac6bmiahyst53ge4g5xltmishrs4h3q2rma":{"value":"bagaaieravto4z7jk4evzpw5uwfjttqweeyg5upno6oxdil5roe3f5pv53jeq","tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq"},"bagaaieraf765opx5uxfpppttpweakkm5qewp5icxktxpc7yxawjbgto55bfa":{"value":"bagaaiera5vlhrcij57bv65swyy2n7aoxaec44q7n3mimmznc7ukttguhu62q","tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq"},"bagaaieraqz4ptlijr3yv2givsljisbaurlbmc2hpjklemidefxcexy5qc76q":{"value":"bagaaieraqbmy7wg4mvfmll4uqjmkucsk4a6ychwrgyrdji6uicfrav7jsqya","tetraplet":"bagaaieratlnmrmeejcaupvdlnzxpfi3iozssmtsenduuooewowvx4qtkgukq"},"bagaaiera25nqqxd2e3nqhchhvqyolu5ezwwysrrvzl7wdnplyojvw7swmv2a":{"value":"bagaaierahbensrh75a4k6yd5bsn2wephkblsflcrwns64grewr3bn5r4kwkq","tetraplet":"bagaaierax2o7eimyin4soeqy5n7sws2htwkkhvdmrfcx75mfof5ct32cvqua"}}}}"#;
static AIR: &'static str = r#"(xor
 (seq
  (seq
   (seq
    (seq
     (seq
      (call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-)
      (new $array-inline
       (seq
        (seq
         (seq
          (seq
           (seq
            (seq
             (ap "12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv" $array-inline)
             (ap "12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU" $array-inline)
            )
            (ap "12D3KooWMMGdfVEJ1rWe1nH1nehYDzNEHhg5ogdfiGk88AupCMnf" $array-inline)
           )
           (ap "12D3KooWJ4bTHirdTFNZpCS72TAzwtdmavTBkkEXtzo6wHL25CtE" $array-inline)
          )
          (ap "12D3KooWAKNos2KogexTXhrkMZzFYpLHuWJ4PgoAhurSAv7o5CWA" $array-inline)
         )
         (ap "12D3KooWDcpWuyrMTDinqNgmXAuRdfd2mTdY9VoXZSAet2pDzh6r" $array-inline)
        )
        (canon %init_peer_id% $array-inline  #array-inline-0)
       )
      )
     )
     (new $dummy
      (seq
       (seq
        (par
         (call %init_peer_id% ("run-console" "print") [#array-inline-0])
         (null)
        )
        (par
         (fold #array-inline-0 peer-0
          (par
           (new $services
            (seq
             (call -relay- ("op" "noop") [])
             (xor
              (seq
               (seq
                (seq
                 (seq
                  (seq
                   (seq
                    (seq
                     (seq
                      (seq
                       (seq
                        (call peer-0 ("decider" "list_get_strings") ["joined_deals"] s)
                        (fold s.$.strings! raw_deal-0
                         (seq
                          (new $aliases
                           (seq
                            (seq
                             (call peer-0 ("json" "parse") [raw_deal-0] deal)
                             (call peer-0 ("srv" "info") [deal.$.spell_id!] spell_info)
                            )
                            (xor
                             (seq
                              (seq
                               (seq
                                (seq
                                 (seq
                                  (seq
                                   (seq
                                    (seq
                                     (seq
                                      (seq
                                       (call spell_info.$.worker_id! ("srv" "list") [] worker_services)
                                       (ap worker_services $services)
                                      )
                                      (fold worker_services service-0
                                       (seq
                                        (xor
                                         (seq
                                          (seq
                                           (seq
                                            (seq
                                             (ap service-0.$.aliases! service-0_flat)
                                             (ap service-0_flat service-0_flat_to_functor)
                                            )
                                            (ap service-0_flat_to_functor.length service-0_flat_length)
                                           )
                                           (call spell_info.$.worker_id! ("cmp" "gt") [service-0_flat_length 0] gt)
                                          )
                                          (match gt true
                                           (ap service-0.$.aliases.[0]! $aliases)
                                          )
                                         )
                                         (ap "NO ALIAS" $aliases)
                                        )
                                        (next service-0)
                                       )
                                      )
                                     )
                                     (par
                                      (seq
                                       (seq
                                        (canon spell_info.$.worker_id! $services  #services_to_functor)
                                        (ap #services_to_functor.length services_length)
                                       )
                                       (call spell_info.$.worker_id! ("debug" "stringify") [services_length] stringify)
                                      )
                                      (seq
                                       (seq
                                        (seq
                                         (ap s.$.strings! s_flat)
                                         (ap s_flat s_flat_to_functor)
                                        )
                                        (ap s_flat_to_functor.length s_flat_length)
                                       )
                                       (call spell_info.$.worker_id! ("debug" "stringify") [s_flat_length] stringify-0)
                                      )
                                     )
                                    )
                                    (call spell_info.$.worker_id! ("op" "concat_strings") [stringify "/" stringify-0 "" "" "" ""] deal_idx)
                                   )
                                   (par
                                    (seq
                                     (canon spell_info.$.worker_id! $aliases  #aliases_canon)
                                     (call spell_info.$.worker_id! ("debug" "stringify") [#aliases_canon] stringify-1)
                                    )
                                    (null)
                                   )
                                  )
                                  (call spell_info.$.worker_id! ("op" "concat_strings") [peer-0 " " deal_idx " @ " deal.$.deal_id! ": " stringify-1] msg)
                                 )
                                 (call peer-0 ("op" "noop") [])
                                )
                                (call -relay- ("op" "noop") [])
                               )
                               (par
                                (call %init_peer_id% ("run-console" "print") [msg])
                                (null)
                               )
                              )
                              (call -relay- ("op" "noop") [])
                             )
                             (seq
                              (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 1])
                              (call -relay- ("op" "noop") [])
                             )
                            )
                           )
                          )
                          (next raw_deal-0)
                         )
                        )
                       )
                       (ap s.$.strings! s_flat-0)
                      )
                      (ap s_flat-0 s_flat-0_to_functor)
                     )
                     (ap s_flat-0_to_functor.length s_flat-0_length)
                    )
                    (call peer-0 ("math" "sub") [s_flat-0_length 1] sub)
                   )
                   (new $services_test
                    (seq
                     (seq
                      (seq
                       (call peer-0 ("math" "add") [sub 1] services_incr)
                       (fold $services services_fold_var
                        (seq
                         (seq
                          (ap services_fold_var $services_test)
                          (canon peer-0 $services_test  #services_iter_canon)
                         )
                         (xor
                          (match #services_iter_canon.length services_incr
                           (null)
                          )
                          (next services_fold_var)
                         )
                        )
                        (never)
                       )
                      )
                      (canon peer-0 $services_test  #services_result_canon)
                     )
                     (ap #services_result_canon services_gate)
                    )
                   )
                  )
                  (ap s.$.strings! s_flat-1)
                 )
                 (ap s_flat-1 s_flat-1_to_functor)
                )
                (ap s_flat-1_to_functor.length s_flat-1_length)
               )
               (call peer-0 ("math" "sub") [s_flat-1_length 1] sub-0)
              )
              (seq
               (call -relay- ("op" "noop") [])
               (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 2])
              )
             )
            )
           )
           (next peer-0)
          )
          (never)
         )
         (null)
        )
       )
       (new $dummy_test
        (seq
         (seq
          (seq
           (call %init_peer_id% ("math" "add") [1 1] dummy_incr)
           (fold $dummy dummy_fold_var
            (seq
             (seq
              (ap dummy_fold_var $dummy_test)
              (canon %init_peer_id% $dummy_test  #dummy_iter_canon)
             )
             (xor
              (match #dummy_iter_canon.length dummy_incr
               (null)
              )
              (next dummy_fold_var)
             )
            )
            (never)
           )
          )
          (canon %init_peer_id% $dummy_test  #dummy_result_canon)
         )
         (ap #dummy_result_canon dummy_gate)
        )
       )
      )
     )
    )
    (call %init_peer_id% ("--after-callback-srv-service--" "console-log") ["OK"])
   )
   (call %init_peer_id% ("--finisher--" "--finish-execution--") [])
  )
  (xor
   (call %init_peer_id% ("callbackSrv" "response") ["ok"])
   (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 3])
  )
 )
 (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 4])
)
"#;

#[test]
fn issue__() {
    // 12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv
    let client_peer_id = "12D3KooW9xAAjbEWMq7VLV3ihLa1dmZBxYS9R4MWNuEzCCZypsw1";
    let mut client_vm = create_avm(unit_call_service(), client_peer_id);

    let prev_data: InterpreterData = serde_json::from_str(PREV_DATA).unwrap();
    // println!("prev_data: {:?}", prev_data);

    let trace = &prev_data.trace;

    println!("trace {} (states_count: {}): [", "prev_data", trace.len());
    for (id, state) in trace.iter().enumerate() {
        println!("  {id}: {state}");
    }
    println!("]");

    let prev_data: Vec<u8> = serde_json::to_vec(&prev_data).unwrap();

    let current_data: InterpreterData = serde_json::from_str(CURRENT_DATA).unwrap();

    let trace = &current_data.trace;
    println!("trace {} (states_count: {}): [", "current_data", trace.len());
    for (id, state) in trace.iter().enumerate() {
        println!("  {id}: {state}");
    }
    println!("]");

    let current_data: Vec<u8> = serde_json::to_vec(&current_data).unwrap();

    let mut call_results = CallResults::new();
    //call_results.insert(70, <_>::default());

    let result = client_vm
        .runner
        .call(
            AIR,
            prev_data,
            current_data,
            "12D3KooW9xAAjbEWMq7VLV3ihLa1dmZBxYS9R4MWNuEzCCZypsw1",
            1677418607051,
            120000,
            None,
            call_results,
        )
        .unwrap();

    //println!("result: {:?}", result);
    assert_eq!(result.ret_code, 0);
}
