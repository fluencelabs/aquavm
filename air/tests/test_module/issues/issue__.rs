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

static PREV_DATA: &'static str = r#"{"version":"0.6.2","interpreter_version":"0.35.1","trace":[{"call":{"executed":{"scalar":"bagaaierahfrmfzj2xvwbiw4oavwpjckj2qfd6qabvs3df6ykotpomtpkfx4a"}}},{"call":{"executed":{"scalar":"bagaaiera2eqrb43xas7wymax6gpplzerf7rtf5jdqxpu6otr7dg66ksijcca"}}},{"call":{"executed":{"scalar":"bagaaieraliozt2hxibf4getmhyka5kigfcax5vn4zn7gahh45lfrn6ao35yq"}}},{"call":{"executed":{"scalar":"bagaaieraiyg6cgb5pko6d2z55p2vuy3fkidqw2ggjtvsui6obnb5u2ella4q"}}},{"call":{"executed":{"scalar":"bagaaierav7oi7mz373dc23sittacoqquchnefdgqurnvc4hity3jyanxsvya"}}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaierawmcvl5iglpvzzzvg6kqkfhajpgctuoqpoxtjxzh5bjsjpus3hrrq","bagaaieralqbjds4aoyvds653n3ua5jzhinevkso6t5gfam4cjrjueugjlreq","bagaaierafjtrewbahe7g5st5pghv2ikq4bodw6fnrh6wgjncvvzqcvhvkoqa","bagaaiera232l6vne3ovygan2kansy42kwpbvbvzcfgzewvagi6ex6cpxzcza","bagaaiera322uxb7emqxsihmqsqvcogiqrf4myfj22oluyhw7fztpr2isq63a"]}},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"executed":{"scalar":"bagaaiera2eqrb43xas7wymax6gpplzerf7rtf5jdqxpu6otr7dg66ksijcca"}}},{"call":{"executed":{"stream":{"cid":"bagaaiera2eqrb43xas7wymax6gpplzerf7rtf5jdqxpu6otr7dg66ksijcca","generation":0}}}},{"call":{"executed":{"scalar":"bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq"}}},{"fold":{"lore":[{"pos":13,"desc":[{"pos":16,"len":2},{"pos":18,"len":0}]}]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaieraay73nl3s2fdyhu3rdper4vub4mnc3jv4i5seaxc3nnftrxwbsfga"]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaieraay73nl3s2fdyhu3rdper4vub4mnc3jv4i5seaxc3nnftrxwbsfga"]}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaieraseifmudiql4gfbcl544rbckmmlg5tp7r4nhgyg2ffaspde6ybrxq","bagaaierafjtrewbahe7g5st5pghv2ikq4bodw6fnrh6wgjncvvzqcvhvkoqa"]}},{"call":{"executed":{"scalar":"bagaaiera2eqrb43xas7wymax6gpplzerf7rtf5jdqxpu6otr7dg66ksijcca"}}},{"call":{"executed":{"scalar":"bagaaieraf5wvnekhwh2udu5m33eljajyxdgcnny4c3jsiqvuyvthtxr7luaa"}}},{"par":[1,4]},{"call":{"executed":{"scalar":"bagaaieravf2khtuqmnv7mmlxgbr3ozs4tj6dlgqoqncw7luupdth4ovrvy6q"}}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaieralqbjds4aoyvds653n3ua5jzhinevkso6t5gfam4cjrjueugjlreq","bagaaierapvawxm33r4sgiwjpuljxawhtehvttioozg6zdhp5m7dwex2wkcnq"]}},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"executed":{"scalar":"bagaaiera2eqrb43xas7wymax6gpplzerf7rtf5jdqxpu6otr7dg66ksijcca"}}},{"call":{"executed":{"scalar":"bagaaieracakynmxsekzjkra7qxoeegzbykkd2ele37ud4u3xfs32zbuam4va"}}},{"call":{"executed":{"scalar":"bagaaierawqtcm73ponlg3nydlf6esctujcrxznfh2bsj4yyliggogaekjonq"}}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaierappxicz7zm36clfgl45lzxoyg3qk2clgkgswlwrdxbebj47agl7tq","bagaaierasrknsei6bsxe2l77ltdv3qqmonoiftjw3ogk5jubb2xfvfbrzwhq"]}},{"call":{"executed":{"scalar":"bagaaiera2eqrb43xas7wymax6gpplzerf7rtf5jdqxpu6otr7dg66ksijcca"}}},{"call":{"executed":{"scalar":"bagaaieraos3ogrlachehp5vapo3meo7dweb6ofyxfdzndjqwmo6oldsuagjq"}}},{"par":[1,4]},{"call":{"executed":{"scalar":"bagaaieravf2khtuqmnv7mmlxgbr3ozs4tj6dlgqoqncw7luupdth4ovrvy6q"}}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaieralqbjds4aoyvds653n3ua5jzhinevkso6t5gfam4cjrjueugjlreq","bagaaierap4hrlxtfmg4uxyphnlgaigenpcpkwe6hogrisxav5fj5bo5ejj5a"]}},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"executed":{"scalar":"bagaaierahhpujisdpq7c66h3mxrudez2hmxwirgfyeepwiigbaku6ll7f7mq"}}},{"call":{"executed":{"scalar":"bagaaieraoxaxxpo5n5sutz4nlkwgs4lhswmnizzhqsm6k3qnalqborrrhhaa"}}},{"call":{"executed":{"scalar":"bagaaieragaooeyyxoy3yassjyzcysj77gt7y7wvlfaetotipb3nn5rcrql7q"}}},{"call":{"executed":{"scalar":"bagaaieraryi7somtsujp6klflfss3tqr7ngzmoaguvx4brafwuoq5hgam2uq"}}},{"call":{"executed":{"stream":{"cid":"bagaaieraen4l4vjpshoyjjd7wg6unuakesxucdcdo4vb47oxynqqqytsfpmq","generation":0}}}},{"call":{"executed":{"scalar":"bagaaieraf3tnagt5opejnb4cv2pfkhga5yk67duwbggwzlo7pr7diej4cnra"}}},{"call":{"executed":{"scalar":"bagaaierac4o4exxjoefdd7djwjbisb22odz6zuxfijdshe3l2lnhahf7ihea"}}},{"call":{"executed":{"scalar":"bagaaiera3xre2bctetigzkdleu3miztskhctp7aacbg4poixilpz64sqzjgq"}}},{"call":{"executed":{"scalar":"bagaaierajdcldxvcq443vkuq36uauln4pyta2f2dltgg273asxqvzpbwxu4q"}}},{"call":{"executed":{"stream":{"cid":"bagaaiera2c5gsog62vmpisjuqqwocactzddp3mpxfsnawarhar4gjbc7noma","generation":1}}}},{"par":[1,0]},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaieraj63rh6tbbyfd3apv4vroky6xcur45dssyopfqb4gipwp6epbmkgq","bagaaieraj7oorumswqqmuomjfbapiavvaashqoeaidjt5x3l3nmf7kyntwmq"]}},{"call":{"executed":{"scalar":"bagaaiera5fltvv2wangpo4dlejkkq5gs5ekli4um4r6sju6jfxc7onllifta"}}},{"call":{"executed":{"scalar":"bagaaieraizjpmil7gtkm4mfq5dznwmc37viqcz3cqx3eb2lysitr2lpryeeq"}}},{"call":{"executed":{"scalar":"bagaaiera2eqrb43xas7wymax6gpplzerf7rtf5jdqxpu6otr7dg66ksijcca"}}},{"call":{"executed":{"scalar":"bagaaiera2azwogmtdzgrjbv77q2cgunk44sc46b6m7qpru4d23tjxyirl6pq"}}},{"call":{"executed":{"scalar":"bagaaierah53ldyomfdmevuo7c66ntndr6bv6mbaw6nsjfpot3ypcryvt3eua"}}},{"call":{"executed":{"scalar":"bagaaierawiaqhw4awfsosudzff4jjo6atlcfislknuht3yia7klwltw44rsq"}}},{"ap":{"gens":[0]}},{"call":{"executed":{"scalar":"bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq"}}},{"fold":{"lore":[{"pos":62,"desc":[{"pos":65,"len":2},{"pos":67,"len":0}]}]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q"]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q"]}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaierahusjlju5gkjv4vmbpgzpuboigaumxnyokrlawjkwp6t2woxylo2q","bagaaiera5nlecg3cp6a3xbjtzuafajo3abe5y6f5wmlyv43usfwlmkxywpra"]}},{"call":{"executed":{"scalar":"bagaaiera2eqrb43xas7wymax6gpplzerf7rtf5jdqxpu6otr7dg66ksijcca"}}},{"call":{"executed":{"scalar":"bagaaierax2uohvl45hvijrpzhki7hgkz3lrf4yxlrvsxlqs5mvoalpr7nwya"}}},{"par":[1,4]},{"call":{"executed":{"scalar":"bagaaieravf2khtuqmnv7mmlxgbr3ozs4tj6dlgqoqncw7luupdth4ovrvy6q"}}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaieralqbjds4aoyvds653n3ua5jzhinevkso6t5gfam4cjrjueugjlreq","bagaaierarccvc6qouxr6ku25xaryla4t65rd2buptw7a5hvreeh5hoksmjaq"]}},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"executed":{"scalar":"bagaaiera2eqrb43xas7wymax6gpplzerf7rtf5jdqxpu6otr7dg66ksijcca"}}},{"call":{"executed":{"scalar":"bagaaieraf27mgtkkspvtf77324czzfgwg3fbfgayihk53tomev6qaubfzswq"}}},{"par":[1,4]},{"call":{"executed":{"scalar":"bagaaieravf2khtuqmnv7mmlxgbr3ozs4tj6dlgqoqncw7luupdth4ovrvy6q"}}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaieralqbjds4aoyvds653n3ua5jzhinevkso6t5gfam4cjrjueugjlreq","bagaaieras3ngmzz3hppxtqldktd7ivz23dy4t2vb2px4cx2darqzs2hmakqa"]}},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"executed":{"scalar":"bagaaieravf2khtuqmnv7mmlxgbr3ozs4tj6dlgqoqncw7luupdth4ovrvy6q"}}},{"call":{"executed":{"scalar":"bagaaiera2rwqugy3qsctgk3q5acfoovmgmmwxsi5p2ztuo5hrfcjcqyqdonq"}}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaierazhexirqprqeklcbntuk522kadql4rn4iotb3tuc3caapov5cg6oa","bagaaierawl7zogbqpui5xbmg4doltmzmljx42nh6tlvnjrvnbu36owskxfjq"]}},{"call":{"executed":{"scalar":"bagaaiera2eqrb43xas7wymax6gpplzerf7rtf5jdqxpu6otr7dg66ksijcca"}}},{"call":{"executed":{"scalar":"bagaaieratb2g7qy4lljvo3hgoiptffmu2n4vqmwpx6gmsoinjildvqy45e3q"}}},{"par":[1,4]},{"call":{"executed":{"scalar":"bagaaieravf2khtuqmnv7mmlxgbr3ozs4tj6dlgqoqncw7luupdth4ovrvy6q"}}},{"ap":{"gens":[0]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta","values":["bagaaieralqbjds4aoyvds653n3ua5jzhinevkso6t5gfam4cjrjueugjlreq","bagaaieraowgjyxbee7kyc7q4s2zkxegtoozt5j7ov3p7d3zovxg4ynb3zmpa"]}},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"call":{"executed":{"scalar":"bagaaierac5ch2oegc3w64kqdehoci7tqqtl3y3jna7toum74jlvhqv6fv6qa"}}},{"call":{"executed":{"scalar":"bagaaieraphdxy2irmwuiboegxwvayhcmbkm4bauskquxm7unchjzegr3udnq"}}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaierakqwml2etrlfjszfvgt7ahshpecxy32i3ylgl3fu7bputyhvrqfea","values":["bagaaiera7f52jngqywgdsrksdgfgrj63zoedyc6wsc7tnw4ivrvofvv67ota"]}},{"call":{"executed":{"scalar":"bagaaiera2fmnh243auuowd4ciyotnbjzyijoatolckg7zqh6mldwuu2o336q"}}},{"call":{"executed":{"scalar":"bagaaierawbpgfg2hzulb23bsvgxwemcpvhhvqlbek6vsal7jy76kt67z2ffq"}}},{"call":{"executed":{"scalar":"bagaaieramzsh5ouimryjitttofln4aiyfd2zb7fl2asljijqlbkwpo7gydrq"}}},{"call":{"executed":{"scalar":"bagaaieraihgrbryxsonssrmmltogowonliu55bogsooqdx2wptqzqsdsdknq"}}},{"call":{"executed":{"scalar":"bagaaieravsec5jogze4whz3zam4cbrthiabadhhp54hvf4eb4m3gmxzczqpa"}}},{"par":[40,0]},{"par":[7,32]},{"call":{"executed":{"scalar":"bagaaierac5ch2oegc3w64kqdehoci7tqqtl3y3jna7toum74jlvhqv6fv6qa"}}},{"call":{"executed":{"scalar":"bagaaieraoilloku5gmmfmhnwdmycrnoehcho7clfgp2ulyb7im3ne7mcyqvq"}}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaieranb5e6zjltfm7fwyin5dqul5si774y5lvxxnyfdct24sjs3lfdahq","values":["bagaaiera7f52jngqywgdsrksdgfgrj63zoedyc6wsc7tnw4ivrvofvv67ota"]}},{"call":{"executed":{"scalar":"bagaaiera5pm3swog7wfrxwbtwbguecosoiywf53rd67sf77vuz57eyq5xlkq"}}},{"ap":{"gens":[4]}},{"call":{"sent_by":"12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv: 69"}},{"par":[7,24]},{"call":{"executed":{"scalar":"bagaaierac5ch2oegc3w64kqdehoci7tqqtl3y3jna7toum74jlvhqv6fv6qa"}}},{"call":{"executed":{"scalar":"bagaaieraoilloku5gmmfmhnwdmycrnoehcho7clfgp2ulyb7im3ne7mcyqvq"}}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaiera5ipjfgyguky65yfcdpydes35effzqohfd44edwuiqxvcmdtce7qa","values":["bagaaiera7f52jngqywgdsrksdgfgrj63zoedyc6wsc7tnw4ivrvofvv67ota"]}},{"call":{"executed":{"scalar":"bagaaiera5pm3swog7wfrxwbtwbguecosoiywf53rd67sf77vuz57eyq5xlkq"}}},{"ap":{"gens":[3]}},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"par":[7,16]},{"call":{"executed":{"scalar":"bagaaierac5ch2oegc3w64kqdehoci7tqqtl3y3jna7toum74jlvhqv6fv6qa"}}},{"call":{"executed":{"scalar":"bagaaieraoilloku5gmmfmhnwdmycrnoehcho7clfgp2ulyb7im3ne7mcyqvq"}}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaierawgmzwvhvwdzx2wm4gigg4hk3i5mo23svrzxked5k3yo34gwt3hrq","values":["bagaaiera7f52jngqywgdsrksdgfgrj63zoedyc6wsc7tnw4ivrvofvv67ota"]}},{"call":{"executed":{"scalar":"bagaaiera5pm3swog7wfrxwbtwbguecosoiywf53rd67sf77vuz57eyq5xlkq"}}},{"ap":{"gens":[1]}},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"par":[7,8]},{"call":{"executed":{"scalar":"bagaaierac5ch2oegc3w64kqdehoci7tqqtl3y3jna7toum74jlvhqv6fv6qa"}}},{"call":{"executed":{"scalar":"bagaaieraoilloku5gmmfmhnwdmycrnoehcho7clfgp2ulyb7im3ne7mcyqvq"}}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaiera74p6pc2zrjq64bhugnqez77rwgpxdrxpzg623sswptc5hfgah2pa","values":["bagaaiera7f52jngqywgdsrksdgfgrj63zoedyc6wsc7tnw4ivrvofvv67ota"]}},{"call":{"executed":{"scalar":"bagaaiera5pm3swog7wfrxwbtwbguecosoiywf53rd67sf77vuz57eyq5xlkq"}}},{"ap":{"gens":[0]}},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"par":[7,0]},{"call":{"executed":{"scalar":"bagaaierac5ch2oegc3w64kqdehoci7tqqtl3y3jna7toum74jlvhqv6fv6qa"}}},{"call":{"executed":{"scalar":"bagaaieraoilloku5gmmfmhnwdmycrnoehcho7clfgp2ulyb7im3ne7mcyqvq"}}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaierasadyr77mawtyya3pa5tuy3jgeq3axev3rn255l5synwgxzctngmq","values":["bagaaiera7f52jngqywgdsrksdgfgrj63zoedyc6wsc7tnw4ivrvofvv67ota"]}},{"call":{"executed":{"scalar":"bagaaiera5pm3swog7wfrxwbtwbguecosoiywf53rd67sf77vuz57eyq5xlkq"}}},{"ap":{"gens":[2]}},{"call":{"executed":{"scalar":"bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta"}}},{"par":[16,1]},{"call":{"executed":{"scalar":"bagaaieral7wowzx7zbxtrwkspbwg22lmphbnxqrz3vhjdndhfhltuj73k7uq"}}},{"call":{"executed":{"scalar":"bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq"}}},{"fold":{"lore":[{"pos":140,"desc":[{"pos":154,"len":2},{"pos":156,"len":0}]},{"pos":132,"desc":[{"pos":156,"len":2},{"pos":158,"len":0}]},{"pos":148,"desc":[{"pos":158,"len":2},{"pos":160,"len":0}]},{"pos":124,"desc":[{"pos":160,"len":2},{"pos":162,"len":0}]},{"pos":116,"desc":[{"pos":162,"len":2},{"pos":164,"len":0}]}]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaierakqwml2etrlfjszfvgt7ahshpecxy32i3ylgl3fu7bputyhvrqfea","values":["bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q"]}},{"ap":{"gens":[1]}},{"canon":{"tetraplet":"bagaaierakqwml2etrlfjszfvgt7ahshpecxy32i3ylgl3fu7bputyhvrqfea","values":["bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q","bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q"]}},{"ap":{"gens":[2]}},{"canon":{"tetraplet":"bagaaierakqwml2etrlfjszfvgt7ahshpecxy32i3ylgl3fu7bputyhvrqfea","values":["bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q","bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q","bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q"]}},{"ap":{"gens":[3]}},{"canon":{"tetraplet":"bagaaierakqwml2etrlfjszfvgt7ahshpecxy32i3ylgl3fu7bputyhvrqfea","values":["bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q","bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q","bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q","bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q"]}},{"ap":{"gens":[4]}},{"canon":{"tetraplet":"bagaaierakqwml2etrlfjszfvgt7ahshpecxy32i3ylgl3fu7bputyhvrqfea","values":["bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q","bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q","bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q","bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q","bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q"]}},{"canon":{"tetraplet":"bagaaierakqwml2etrlfjszfvgt7ahshpecxy32i3ylgl3fu7bputyhvrqfea","values":["bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q"]}},{"call":{"executed":{"scalar":"bagaaieral7wowzx7zbxtrwkspbwg22lmphbnxqrz3vhjdndhfhltuj73k7uq"}}},{"ap":{"gens":[0]}},{"call":{"sent_by":"12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv: 60"}},{"call":{"executed":{"scalar":"bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq"}}},{"fold":{"lore":[{"pos":166,"desc":[{"pos":170,"len":2},{"pos":172,"len":0}]}]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaierakqwml2etrlfjszfvgt7ahshpecxy32i3ylgl3fu7bputyhvrqfea","values":["bagaaierat7fo6omti6xhbeoxvhylphnmvqqpx7iipwohbbmdimnih3dfayqa"]}},{"canon":{"tetraplet":"bagaaierakqwml2etrlfjszfvgt7ahshpecxy32i3ylgl3fu7bputyhvrqfea","values":["bagaaierat7fo6omti6xhbeoxvhylphnmvqqpx7iipwohbbmdimnih3dfayqa"]}},{"ap":{"gens":[0]}},{"call":{"executed":{"scalar":"bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq"}}},{"fold":{"lore":[{"pos":173,"desc":[{"pos":176,"len":2},{"pos":178,"len":0}]}]}},{"ap":{"gens":[0]}},{"canon":{"tetraplet":"bagaaierakqwml2etrlfjszfvgt7ahshpecxy32i3ylgl3fu7bputyhvrqfea","values":["bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q"]}},{"canon":{"tetraplet":"bagaaierakqwml2etrlfjszfvgt7ahshpecxy32i3ylgl3fu7bputyhvrqfea","values":["bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q"]}},{"ap":{"gens":[0]}}],"streams":{},"r_streams":{"$array-inline-7":{"4395":[1]},"$option-inline":{"14326":[1]},"$array-inline-8":{"4783":[1]},"$status-0":{"15692":[1]},"$worker_spell":{"933":[1]},"$array-inline-16":{"10156":[1]},"$array-inline-15":{"9748":[1]},"$hashes":{"6013":[2]},"$result-2":{"15707":[1]},"$result_test":{"7294":[1]},"$status-0_test":{"16333":[1]},"$cre_errs":{"967":[0]},"$array-inline":{"615":[1]},"$array-inline-10":{"5725":[1]},"$result":{"6727":[1]},"$worker_spell_test":{"2551":[1]},"$array-inline-19":{"11348":[1]},"$array-inline-24":{"13400":[1]},"$successful_test":{"15797":[5]},"$array-inline-9":{"5327":[1]},"$result-2_test":{"16799":[1]},"$successful":{"14140":[5]},"$option-inline-1":{"15076":[1,1,1,1,1]},"$array-inline-25":{"13820":[1]},"$subnetwork_id":{"14157":[1]},"$reg_errs":{"952":[0]}},"lcid":69,"cid_info":{"value_store":{"bagaaieraww7kig3mmi7xycprx4snzlsy5ovtydg5scwzm26ehjc3isdh4evq":true,"bagaaiera2rwqugy3qsctgk3q5acfoovmgmmwxsi5p2ztuo5hrfcjcqyqdonq":{"absent":false,"error":"","str":"install finished","success":true},"bagaaieraos3ogrlachehp5vapo3meo7dweb6ofyxfdzndjqwmo6oldsuagjq":"[\"parsed worker definition\",{\"services\":[{\"modules\":[{\"config\":\"QmVGVJDf2ToECPcSne8e5vGDYkmwaaw7dkJAwXmtUeeirJ\",\"wasm\":\"Qme7EFZ9Y6b32o3hoWtM1N8yqCWm568DU8bKt9bhFKgsAa\"},{\"config\":\"QmdMzTeP27iZc3Bv6doWLnt9dBRDXJtZxNafFFXxB4vg2F\",\"wasm\":\"QmeXQZzA3hYbDJgKefZfSQfu1tjN15HFxeErEGoo8V9ebf\"}],\"name\":\"filesys_adapter\"}],\"spells\":[]}]","bagaaierajdcldxvcq443vkuq36uauln4pyta2f2dltgg273asxqvzpbwxu4q":"33fde9e70239f696962746b787db472ea73b9ca4360f5f81c9be52f25131d991","bagaaierasxfswhclnibi32wz7ghx7zawdnzsoa5ncus6fhompv5pf47jnwra":"Installing worker for deal","bagaaiera2c5gsog62vmpisjuqqwocactzddp3mpxfsnawarhar4gjbc7noma":"hash:33fde9e70239f696962746b787db472ea73b9ca4360f5f81c9be52f25131d991","bagaaieral7wowzx7zbxtrwkspbwg22lmphbnxqrz3vhjdndhfhltuj73k7uq":0,"bagaaierawqtcm73ponlg3nydlf6esctujcrxznfh2bsj4yyliggogaekjonq":{"services":[{"modules":[{"config":"QmVGVJDf2ToECPcSne8e5vGDYkmwaaw7dkJAwXmtUeeirJ","wasm":"Qme7EFZ9Y6b32o3hoWtM1N8yqCWm568DU8bKt9bhFKgsAa"},{"config":"QmdMzTeP27iZc3Bv6doWLnt9dBRDXJtZxNafFFXxB4vg2F","wasm":"QmeXQZzA3hYbDJgKefZfSQfu1tjN15HFxeErEGoo8V9ebf"}],"name":"filesys_adapter"}],"spells":[]},"bagaaierah53ldyomfdmevuo7c66ntndr6bv6mbaw6nsjfpot3ypcryvt3eua":"blueprint:filesys_adapter","bagaaieradk7awvpox6hvf22l6jw7ld5dm6ycjnsrzbeheenthjy3gumtq7ia":["parsed worker definition",{"services":[{"modules":[{"config":"QmVGVJDf2ToECPcSne8e5vGDYkmwaaw7dkJAwXmtUeeirJ","wasm":"Qme7EFZ9Y6b32o3hoWtM1N8yqCWm568DU8bKt9bhFKgsAa"},{"config":"QmdMzTeP27iZc3Bv6doWLnt9dBRDXJtZxNafFFXxB4vg2F","wasm":"QmeXQZzA3hYbDJgKefZfSQfu1tjN15HFxeErEGoo8V9ebf"}],"name":"filesys_adapter"}],"spells":[]}],"bagaaieranodle477gt6odhllqbhp6wr7k5d23jhkuixr2soadzjn3n4hlnfq":1,"bagaaieraf27mgtkkspvtf77324czzfgwg3fbfgayihk53tomev6qaubfzswq":"\"Installation finished\"","bagaaieraoxaxxpo5n5sutz4nlkwgs4lhswmnizzhqsm6k3qnalqborrrhhaa":{"contents":"{\"name\":\"filesys_adapter\",\"mapped_dirs\":[[\"sites\",\"./tmp\"]],\"preopened_files\":[\"./tmp\"]}","error":"","success":true},"bagaaierax2uohvl45hvijrpzhki7hgkz3lrf4yxlrvsxlqs5mvoalpr7nwya":"[\"filesys_adapter\",\"is already deployed, doing nothing\"]","bagaaierazyw4d3nzn5x3cxwj7hxzuf23pcxeyifdzyohsjeosz6xe55f7bsa":"is already deployed, doing nothing","bagaaierais767rljtx6pgfls37x53uwudzu5wqbo5cvkbguswemaskvjmnca":["Installing worker for deal","44f241111aa103de816f778e86591da404921bf1"],"bagaaieramzsh5ouimryjitttofln4aiyfd2zb7fl2asljijqlbkwpo7gydrq":"J3kKhyr5fPWb5ZgiEbmmeUoAdeyyR4hMkxES7eBrqHcz","bagaaieracakynmxsekzjkra7qxoeegzbykkd2ele37ud4u3xfs32zbuam4va":{"contents":"{\"services\":[{\"modules\":[{\"config\":\"QmVGVJDf2ToECPcSne8e5vGDYkmwaaw7dkJAwXmtUeeirJ\",\"wasm\":\"Qme7EFZ9Y6b32o3hoWtM1N8yqCWm568DU8bKt9bhFKgsAa\"},{\"config\":\"QmdMzTeP27iZc3Bv6doWLnt9dBRDXJtZxNafFFXxB4vg2F\",\"wasm\":\"QmeXQZzA3hYbDJgKefZfSQfu1tjN15HFxeErEGoo8V9ebf\"}],\"name\":\"filesys_adapter\"}],\"spells\":[]}","error":"","success":true},"bagaaiera2eqrb43xas7wymax6gpplzerf7rtf5jdqxpu6otr7dg66ksijcca":"fec052ac-e2d2-42e7-812b-80a1ba2e0960","bagaaierav7oi7mz373dc23sittacoqquchnefdgqurnvc4hity3jyanxsvya":"QmeUuGaDW8gUoJRFtY6uH3ACs7nzuQcK7cpxT6pjo1M8Do","bagaaieraizjpmil7gtkm4mfq5dznwmc37viqcz3cqx3eb2lysitr2lpryeeq":"36eba0bc4480b27452e6dd796b320994fd61f8c98b1e4374a0afeb82f1792183","bagaaieran6h3wwgrdxsa2ujpkcplufkza4nsoppdndh4udywqltatq2csxpa":["Worker installation finished with status","install finished"],"bagaaieraf5wvnekhwh2udu5m33eljajyxdgcnny4c3jsiqvuyvthtxr7luaa":"[\"Installing worker for deal\",\"44f241111aa103de816f778e86591da404921bf1\"]","bagaaierac5ch2oegc3w64kqdehoci7tqqtl3y3jna7toum74jlvhqv6fv6qa":1677418626,"bagaaieraxffpbdj2ec6gnowcboaynv4vlunqplgxgqtmfwjrtbo3e7xt5nlq":"parsed worker definition","bagaaierackxdfsy6yawqd3ndlanre7a75y5q3rjvolwwxlzds4q2apmc4eta":"","bagaaieratb2g7qy4lljvo3hgoiptffmu2n4vqmwpx6gmsoinjildvqy45e3q":"[\"Worker installation finished with status\",\"install finished\"]","bagaaierax26wdkngwyjrnw2yxubfzapimtf52de34nne56geg3he4ypb343a":["filesys_adapter","is already deployed, doing nothing"],"bagaaieraliozt2hxibf4getmhyka5kigfcax5vn4zn7gahh45lfrn6ao35yq":"/dns4/ipfs.fluence.dev/tcp/5001","bagaaierausvi7zskferr5dgefq26htqu3d3zztmxitmzbml6neweelz5idia":"Installation finished","bagaaiera5pm3swog7wfrxwbtwbguecosoiywf53rd67sf77vuz57eyq5xlkq":{"error":"","key_id":"J3kKhyr5fPWb5ZgiEbmmeUoAdeyyR4hMkxES7eBrqHcz","success":true},"bagaaieravf2khtuqmnv7mmlxgbr3ozs4tj6dlgqoqncw7luupdth4ovrvy6q":{"error":"","success":true},"bagaaierahfrmfzj2xvwbiw4oavwpjckj2qfd6qabvs3df6ykotpomtpkfx4a":"12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv","bagaaiera7tljubuuqenj5z5rzxn224vvdkeirk2ja4kkfar7iralnu3vfqza":"filesys_adapter","bagaaierawiaqhw4awfsosudzff4jjo6atlcfislknuht3yia7klwltw44rsq":{"absent":false,"error":"","str":"36eba0bc4480b27452e6dd796b320994fd61f8c98b1e4374a0afeb82f1792183","success":true},"bagaaieraryi7somtsujp6klflfss3tqr7ngzmoaguvx4brafwuoq5hgam2uq":"7ae5aa823730b505698af30cbfd8a239d1e4add14b2b6cbc02d9ed68ab8a55f0","bagaaieraiyg6cgb5pko6d2z55p2vuy3fkidqw2ggjtvsui6obnb5u2ella4q":"44f241111aa103de816f778e86591da404921bf1","bagaaieraoilloku5gmmfmhnwdmycrnoehcho7clfgp2ulyb7im3ne7mcyqvq":{"error":"","peer_id":"12D3KooWA4Xop1JaT3MHxwYMkCepYsv4iPVopMXwCz5iHYdBfeSB","success":true,"weight":0},"bagaaieraphdxy2irmwuiboegxwvayhcmbkm4bauskquxm7unchjzegr3udnq":"12D3KooWA4Xop1JaT3MHxwYMkCepYsv4iPVopMXwCz5iHYdBfeSB","bagaaiera3xre2bctetigzkdleu3miztskhctp7aacbg4poixilpz64sqzjgq":{"name":"use_filesys"},"bagaaieraf3tnagt5opejnb4cv2pfkhga5yk67duwbggwzlo7pr7diej4cnra":{"error":"","path":"/tmp/vault/spell_fec052ac-e2d2-42e7-812b-80a1ba2e0960_1366/QmeXQZzA3hYbDJgKefZfSQfu1tjN15HFxeErEGoo8V9ebf","success":true},"bagaaieravsec5jogze4whz3zam4cbrthiabadhhp54hvf4eb4m3gmxzczqpa":["12D3KooWMMGdfVEJ1rWe1nH1nehYDzNEHhg5ogdfiGk88AupCMnf","12D3KooWJ4bTHirdTFNZpCS72TAzwtdmavTBkkEXtzo6wHL25CtE","12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU","12D3KooWDcpWuyrMTDinqNgmXAuRdfd2mTdY9VoXZSAet2pDzh6r","12D3KooWAKNos2KogexTXhrkMZzFYpLHuWJ4PgoAhurSAv7o5CWA"],"bagaaierayo6sno4aybqjpm7kj6awxjbwbszpydosqlpqcs3in6iyqjsepi2a":"install finished","bagaaierae5a6y24zmdxpx4j6ax24ee33xkbdwgqlhhamu2a6wacc3pfxq3ya":"Worker installation finished with status","bagaaieragaooeyyxoy3yassjyzcysj77gt7y7wvlfaetotipb3nn5rcrql7q":{"mapped_dirs":[["sites","./tmp"]],"name":"filesys_adapter","preopened_files":["./tmp"]},"bagaaiera2fmnh243auuowd4ciyotnbjzyijoatolckg7zqh6mldwuu2o336q":[251,108,149,231,124,217,234,236,153,159,51,220,155,40,175,15,81,122,186,178,16,118,210,149,147,12,180,18,190,164,51,143],"bagaaiera5fltvv2wangpo4dlejkkq5gs5ekli4um4r6sju6jfxc7onllifta":{"dependencies":["hash:7ae5aa823730b505698af30cbfd8a239d1e4add14b2b6cbc02d9ed68ab8a55f0","hash:33fde9e70239f696962746b787db472ea73b9ca4360f5f81c9be52f25131d991"],"name":"filesys_adapter"},"bagaaierawbpgfg2hzulb23bsvgxwemcpvhhvqlbek6vsal7jy76kt67z2ffq":{"error":[],"signature":[[198,148,182,233,133,103,177,157,220,9,229,24,175,44,6,239,190,241,210,138,168,134,230,147,130,109,214,85,160,77,54,23,147,132,82,42,75,240,142,55,249,93,70,44,90,44,85,15,179,16,191,14,67,110,21,162,132,225,111,244,188,130,61,3]],"success":true},"bagaaieracy3pt3ghwwakozisphlya7ms3wuarosy2unlvz6rgmxl4hlchttq":"worker","bagaaierac4o4exxjoefdd7djwjbisb22odz6zuxfijdshe3l2lnhahf7ihea":{"contents":"{\"name\":\"use_filesys\"}","error":"","success":true},"bagaaieraihgrbryxsonssrmmltogowonliu55bogsooqdx2wptqzqsdsdknq":"R8H5vcYLYX9CMWRxnjWV7zNJu22ZoLCcKZyg7noiCfbVsox8AyaZJLYJjFhs","bagaaieraysfvwguxo3eemaw6emdnpeb2ojarlcsqo7t2qum265odgra3qm2a":"ok","bagaaierahhpujisdpq7c66h3mxrudez2hmxwirgfyeepwiigbaku6ll7f7mq":{"error":"","path":"/tmp/vault/spell_fec052ac-e2d2-42e7-812b-80a1ba2e0960_1366/Qme7EFZ9Y6b32o3hoWtM1N8yqCWm568DU8bKt9bhFKgsAa","success":true},"bagaaiera2azwogmtdzgrjbv77q2cgunk44sc46b6m7qpru4d23tjxyirl6pq":{"error":"","flag":true,"success":true},"bagaaieraen4l4vjpshoyjjd7wg6unuakesxucdcdo4vb47oxynqqqytsfpmq":"hash:7ae5aa823730b505698af30cbfd8a239d1e4add14b2b6cbc02d9ed68ab8a55f0"},"tetraplet_store":{"bagaaiera74p6pc2zrjq64bhugnqez77rwgpxdrxpzg623sswptc5hfgah2pa":{"peer_pk":"12D3KooWDcpWuyrMTDinqNgmXAuRdfd2mTdY9VoXZSAet2pDzh6r","service_id":"","function_name":"","json_path":""},"bagaaieranb5e6zjltfm7fwyin5dqul5si774y5lvxxnyfdct24sjs3lfdahq":{"peer_pk":"12D3KooWMMGdfVEJ1rWe1nH1nehYDzNEHhg5ogdfiGk88AupCMnf","service_id":"","function_name":"","json_path":""},"bagaaiera5ipjfgyguky65yfcdpydes35effzqohfd44edwuiqxvcmdtce7qa":{"peer_pk":"12D3KooWJ4bTHirdTFNZpCS72TAzwtdmavTBkkEXtzo6wHL25CtE","service_id":"","function_name":"","json_path":""},"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta":{"peer_pk":"12D3KooW9xAAjbEWMq7VLV3ihLa1dmZBxYS9R4MWNuEzCCZypsw1","service_id":"","function_name":"","json_path":""},"bagaaieraw7xvcjfrfpvhezpg6jwyax4g62he6ao3xa2r7wg6k2pbs4rm5zia":{"peer_pk":"12D3KooW9xAAjbEWMq7VLV3ihLa1dmZBxYS9R4MWNuEzCCZypsw1","service_id":"getDataSrv","function_name":"spell_id","json_path":""},"bagaaiera4iva2sfuxxv25zpxhbs6j34psy6okf4ivatfn4ibeiet7qj3dtbq":{"peer_pk":"12D3KooW9xAAjbEWMq7VLV3ihLa1dmZBxYS9R4MWNuEzCCZypsw1","service_id":"json","function_name":"parse","json_path":""},"bagaaieraxixqolihqssnzks5knefytz6mw2af6m7d7nq6drh73qqe4zz6rrq":{"peer_pk":"12D3KooW9xAAjbEWMq7VLV3ihLa1dmZBxYS9R4MWNuEzCCZypsw1","service_id":"getDataSrv","function_name":"deal_id","json_path":""},"bagaaierasadyr77mawtyya3pa5tuy3jgeq3axev3rn255l5synwgxzctngmq":{"peer_pk":"12D3KooWAKNos2KogexTXhrkMZzFYpLHuWJ4PgoAhurSAv7o5CWA","service_id":"","function_name":"","json_path":""},"bagaaieraokkcckkr76xf2mxv7bi7sidjgsetnvlt4bzpgor4dancal4y64za":{"peer_pk":"12D3KooW9xAAjbEWMq7VLV3ihLa1dmZBxYS9R4MWNuEzCCZypsw1","service_id":"fec052ac-e2d2-42e7-812b-80a1ba2e0960","function_name":"get_string","json_path":".$.str"},"bagaaierabdssmy26apvur2esowv2kcl3sjdbvi7ayfuwbi3e3bdvoathwwja":{"peer_pk":"12D3KooW9xAAjbEWMq7VLV3ihLa1dmZBxYS9R4MWNuEzCCZypsw1","service_id":"op","function_name":"concat_strings","json_path":""},"bagaaierakutsoqdgbi3l6xyxhqnjpevqpgzqkkhfseztjpcyxoadpc7dhjtq":{"peer_pk":"12D3KooW9xAAjbEWMq7VLV3ihLa1dmZBxYS9R4MWNuEzCCZypsw1","service_id":"json","function_name":"parse","json_path":".$.services.$.name"},"bagaaierakqwml2etrlfjszfvgt7ahshpecxy32i3ylgl3fu7bputyhvrqfea":{"peer_pk":"12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv","service_id":"","function_name":"","json_path":""},"bagaaieral7eryqn2i4wqftwyf54g3h754ahycsjxvlmwmhaszh2oavlc6c5q":{"peer_pk":"12D3KooW9xAAjbEWMq7VLV3ihLa1dmZBxYS9R4MWNuEzCCZypsw1","service_id":"getDataSrv","function_name":"ipfs","json_path":""},"bagaaieraqqzeaizwp6ps4zwtih2b7hxausdrwb5nrg64m5mwnmszeezwg7xq":{"peer_pk":"12D3KooW9xAAjbEWMq7VLV3ihLa1dmZBxYS9R4MWNuEzCCZypsw1","service_id":"getDataSrv","function_name":"worker_def_cid","json_path":""},"bagaaierauc5xmwuxsezye7he22l7l4pl4d2ye3aedxnvsz7b7wrzryysu7qa":{"peer_pk":"12D3KooW9xAAjbEWMq7VLV3ihLa1dmZBxYS9R4MWNuEzCCZypsw1","service_id":"srv","function_name":"resolve_alias","json_path":""},"bagaaierawgmzwvhvwdzx2wm4gigg4hk3i5mo23svrzxked5k3yo34gwt3hrq":{"peer_pk":"12D3KooWMigkP4jkVyufq5JnDJL6nXvyjeaDNpRfEZqQhsG3sYCU","service_id":"","function_name":"","json_path":""},"bagaaiera44klezs6xrzpsmtnsg6vw25njkd37gkybe66hqmngs5re6i57ctq":{"peer_pk":"12D3KooWHCJbJKGDfCgHSoCuK9q4STyRnVveqLoXAPBbXHTZx9Cv","service_id":"insecure_sig","function_name":"get_peer_id","json_path":""}},"canon_store":{"bagaaieraowgjyxbee7kyc7q4s2zkxegtoozt5j7ov3p7d3zovxg4ynb3zmpa":{"value":"bagaaieran6h3wwgrdxsa2ujpkcplufkza4nsoppdndh4udywqltatq2csxpa","tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta"},"bagaaiera7f52jngqywgdsrksdgfgrj63zoedyc6wsc7tnw4ivrvofvv67ota":{"value":"bagaaieraphdxy2irmwuiboegxwvayhcmbkm4bauskquxm7unchjzegr3udnq","tetraplet":"bagaaiera44klezs6xrzpsmtnsg6vw25njkd37gkybe66hqmngs5re6i57ctq"},"bagaaieralqbjds4aoyvds653n3ua5jzhinevkso6t5gfam4cjrjueugjlreq":{"value":"bagaaiera2eqrb43xas7wymax6gpplzerf7rtf5jdqxpu6otr7dg66ksijcca","tetraplet":"bagaaieraw7xvcjfrfpvhezpg6jwyax4g62he6ao3xa2r7wg6k2pbs4rm5zia"},"bagaaierat7fo6omti6xhbeoxvhylphnmvqqpx7iipwohbbmdimnih3dfayqa":{"value":"bagaaieraysfvwguxo3eemaw6emdnpeb2ojarlcsqo7t2qum265odgra3qm2a","tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta"},"bagaaieraay73nl3s2fdyhu3rdper4vub4mnc3jv4i5seaxc3nnftrxwbsfga":{"value":"bagaaiera2eqrb43xas7wymax6gpplzerf7rtf5jdqxpu6otr7dg66ksijcca","tetraplet":"bagaaierauc5xmwuxsezye7he22l7l4pl4d2ye3aedxnvsz7b7wrzryysu7qa"},"bagaaierarccvc6qouxr6ku25xaryla4t65rd2buptw7a5hvreeh5hoksmjaq":{"value":"bagaaierax26wdkngwyjrnw2yxubfzapimtf52de34nne56geg3he4ypb343a","tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta"},"bagaaiera322uxb7emqxsihmqsqvcogiqrf4myfj22oluyhw7fztpr2isq63a":{"value":"bagaaieraliozt2hxibf4getmhyka5kigfcax5vn4zn7gahh45lfrn6ao35yq","tetraplet":"bagaaieral7eryqn2i4wqftwyf54g3h754ahycsjxvlmwmhaszh2oavlc6c5q"},"bagaaieraj63rh6tbbyfd3apv4vroky6xcur45dssyopfqb4gipwp6epbmkgq":{"value":"bagaaieraen4l4vjpshoyjjd7wg6unuakesxucdcdo4vb47oxynqqqytsfpmq","tetraplet":"bagaaierabdssmy26apvur2esowv2kcl3sjdbvi7ayfuwbi3e3bdvoathwwja"},"bagaaierawmcvl5iglpvzzzvg6kqkfhajpgctuoqpoxtjxzh5bjsjpus3hrrq":{"value":"bagaaieracy3pt3ghwwakozisphlya7ms3wuarosy2unlvz6rgmxl4hlchttq","tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta"},"bagaaierapvawxm33r4sgiwjpuljxawhtehvttioozg6zdhp5m7dwex2wkcnq":{"value":"bagaaierais767rljtx6pgfls37x53uwudzu5wqbo5cvkbguswemaskvjmnca","tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta"},"bagaaierahusjlju5gkjv4vmbpgzpuboigaumxnyokrlawjkwp6t2woxylo2q":{"value":"bagaaiera7tljubuuqenj5z5rzxn224vvdkeirk2ja4kkfar7iralnu3vfqza","tetraplet":"bagaaierakutsoqdgbi3l6xyxhqnjpevqpgzqkkhfseztjpcyxoadpc7dhjtq"},"bagaaieraj7oorumswqqmuomjfbapiavvaashqoeaidjt5x3l3nmf7kyntwmq":{"value":"bagaaiera2c5gsog62vmpisjuqqwocactzddp3mpxfsnawarhar4gjbc7noma","tetraplet":"bagaaierabdssmy26apvur2esowv2kcl3sjdbvi7ayfuwbi3e3bdvoathwwja"},"bagaaieraseifmudiql4gfbcl544rbckmmlg5tp7r4nhgyg2ffaspde6ybrxq":{"value":"bagaaierasxfswhclnibi32wz7ghx7zawdnzsoa5ncus6fhompv5pf47jnwra","tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta"},"bagaaiera5nlecg3cp6a3xbjtzuafajo3abe5y6f5wmlyv43usfwlmkxywpra":{"value":"bagaaierazyw4d3nzn5x3cxwj7hxzuf23pcxeyifdzyohsjeosz6xe55f7bsa","tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta"},"bagaaierappxicz7zm36clfgl45lzxoyg3qk2clgkgswlwrdxbebj47agl7tq":{"value":"bagaaieraxffpbdj2ec6gnowcboaynv4vlunqplgxgqtmfwjrtbo3e7xt5nlq","tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta"},"bagaaierawl7zogbqpui5xbmg4doltmzmljx42nh6tlvnjrvnbu36owskxfjq":{"value":"bagaaierayo6sno4aybqjpm7kj6awxjbwbszpydosqlpqcs3in6iyqjsepi2a","tetraplet":"bagaaieraokkcckkr76xf2mxv7bi7sidjgsetnvlt4bzpgor4dancal4y64za"},"bagaaieras3ngmzz3hppxtqldktd7ivz23dy4t2vb2px4cx2darqzs2hmakqa":{"value":"bagaaierausvi7zskferr5dgefq26htqu3d3zztmxitmzbml6neweelz5idia","tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta"},"bagaaierazhexirqprqeklcbntuk522kadql4rn4iotb3tuc3caapov5cg6oa":{"value":"bagaaierae5a6y24zmdxpx4j6ax24ee33xkbdwgqlhhamu2a6wacc3pfxq3ya","tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta"},"bagaaierafjtrewbahe7g5st5pghv2ikq4bodw6fnrh6wgjncvvzqcvhvkoqa":{"value":"bagaaieraiyg6cgb5pko6d2z55p2vuy3fkidqw2ggjtvsui6obnb5u2ella4q","tetraplet":"bagaaieraxixqolihqssnzks5knefytz6mw2af6m7d7nq6drh73qqe4zz6rrq"},"bagaaierap4hrlxtfmg4uxyphnlgaigenpcpkwe6hogrisxav5fj5bo5ejj5a":{"value":"bagaaieradk7awvpox6hvf22l6jw7ld5dm6ycjnsrzbeheenthjy3gumtq7ia","tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta"},"bagaaiera232l6vne3ovygan2kansy42kwpbvbvzcfgzewvagi6ex6cpxzcza":{"value":"bagaaierav7oi7mz373dc23sittacoqquchnefdgqurnvc4hity3jyanxsvya","tetraplet":"bagaaieraqqzeaizwp6ps4zwtih2b7hxausdrwb5nrg64m5mwnmszeezwg7xq"},"bagaaierasrknsei6bsxe2l77ltdv3qqmonoiftjw3ogk5jubb2xfvfbrzwhq":{"value":"bagaaierawqtcm73ponlg3nydlf6esctujcrxznfh2bsj4yyliggogaekjonq","tetraplet":"bagaaiera4iva2sfuxxv25zpxhbs6j34psy6okf4ivatfn4ibeiet7qj3dtbq"},"bagaaieravtfssvzryvy66sfwmwpzu7bdwf6rozfnku5z63inrpeoquoim42q":{"value":"bagaaieraww7kig3mmi7xycprx4snzlsy5ovtydg5scwzm26ehjc3isdh4evq","tetraplet":"bagaaieraq57pzrgr4jb6jbz74o5fc3u7xmpbreki4y3otzbyovla3ptxosta"}}}}"#;
static AIR: &'static str = r#"(xor
(seq
(seq
(seq
(seq
(seq
(seq
(seq
(call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id)
)
(call %init_peer_id% ("getDataSrv" "ipfs") [] ipfs)
)
(call %init_peer_id% ("getDataSrv" "deal_id") [] deal_id)
)
(call %init_peer_id% ("getDataSrv" "worker_def_cid") [] worker_def_cid)
)
(new $array-inline
(seq
(seq
(seq
(seq
(seq
(ap "worker" $array-inline)
(ap spell_id $array-inline)
)
(ap deal_id $array-inline)
)
(ap worker_def_cid $array-inline)
)
(ap ipfs $array-inline)
)
(canon %init_peer_id% $array-inline #array-inline-0)
)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-0])
)
(new $worker_spell
(new $reg_errs
(new $cre_errs
(seq
(seq
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-1)
(xor
(call %init_peer_id% ("srv" "resolve_alias") ["worker-spell"] $worker_spell)
(seq
(ap spell_id-1 $worker_spell)
(xor
(seq
(seq
(seq
(seq
(call %init_peer_id% ("srv" "add_alias") ["worker-spell" spell_id-1])
(new $array-inline-1
(seq
(seq
(ap "alias created 'worker-spell'" $array-inline-1)
(ap spell_id-1 $array-inline-1)
)
(canon %init_peer_id% $array-inline-1 #array-inline-1-0)
)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-2)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-1-0] str)
)
(par
(call %init_peer_id% (spell_id-2 "list_push_string") ["logs" str])
(seq
(new $array-inline-2
(seq
(seq
(ap spell_id-2 $array-inline-2)
(ap #array-inline-1-0 $array-inline-2)
)
(canon %init_peer_id% $array-inline-2 #array-inline-2-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-2-0])
)
)
)
(seq
(seq
(seq
(new $array-inline-3
(seq
(seq
(ap "error creating alias" $array-inline-3)
(ap %last_error% $array-inline-3)
)
(canon %init_peer_id% $array-inline-3 #array-inline-3-0)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-3)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-3-0] str-0)
)
(par
(call %init_peer_id% (spell_id-3 "list_push_string") ["logs" str-0])
(seq
(new $array-inline-4
(seq
(seq
(ap spell_id-3 $array-inline-4)
(ap #array-inline-3-0 $array-inline-4)
)
(canon %init_peer_id% $array-inline-4 #array-inline-4-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-4-0])
)
)
)
)
)
)
)
(xor
(seq
(new $worker_spell_test
(seq
(seq
(seq
(call %init_peer_id% ("math" "add") [0 1] worker_spell_incr)
(fold $worker_spell s
(seq
(seq
(ap s $worker_spell_test)
(canon %init_peer_id% $worker_spell_test #worker_spell_iter_canon)
)
(xor
(match #worker_spell_iter_canon.length worker_spell_incr
(null)
)
(next s)
)
)
(never)
)
)
(canon %init_peer_id% $worker_spell_test #worker_spell_result_canon)
)
(ap #worker_spell_result_canon worker_spell_gate)
)
)
(mismatch worker_spell_gate.$.[0]! spell_id-1
(xor
(seq
(seq
(seq
(new $array-inline-5
(seq
(seq
(seq
(seq
(ap "Another worker spell is deployed on this worker (existing, current)" $array-inline-5)
(new $worker_spell_test-0
(seq
(seq
(seq
(call %init_peer_id% ("math" "add") [0 1] worker_spell_incr-0)
(fold $worker_spell s
(seq
(seq
(ap s $worker_spell_test-0)
(canon %init_peer_id% $worker_spell_test-0 #worker_spell_iter_canon-0)
)
(xor
(match #worker_spell_iter_canon-0.length worker_spell_incr-0
(null)
)
(next s)
)
)
(never)
)
)
(canon %init_peer_id% $worker_spell_test-0 #worker_spell_result_canon-0)
)
(ap #worker_spell_result_canon-0 worker_spell_gate-0)
)
)
)
(ap worker_spell_gate-0.$.[0]! $array-inline-5)
)
(ap spell_id-1 $array-inline-5)
)
(canon %init_peer_id% $array-inline-5 #array-inline-5-0)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-4)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-5-0] str-1)
)
(par
(call %init_peer_id% (spell_id-4 "list_push_string") ["logs" str-1])
(seq
(new $array-inline-6
(seq
(seq
(ap spell_id-4 $array-inline-6)
(ap #array-inline-5-0 $array-inline-6)
)
(canon %init_peer_id% $array-inline-6 #array-inline-6-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-6-0])
)
)
)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 1])
)
)
)
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
(seq
(new $array-inline-7
(seq
(seq
(ap "Installing worker for deal" $array-inline-7)
(ap deal_id $array-inline-7)
)
(canon %init_peer_id% $array-inline-7 #array-inline-7-0)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-5)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-7-0] str-2)
)
(par
(call %init_peer_id% (spell_id-5 "list_push_string") ["logs" str-2])
(seq
(new $array-inline-8
(seq
(seq
(ap spell_id-5 $array-inline-8)
(ap #array-inline-7-0 $array-inline-8)
)
(canon %init_peer_id% $array-inline-8 #array-inline-8-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-8-0])
)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-6)
)
(xor
(seq
(call %init_peer_id% ("aqua-ipfs" "cat_from") [worker_def_cid ipfs] json)
(xor
(match json.$.success! true
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
(call %init_peer_id% ("json" "parse") [json.$.contents!] worker_definition)
(new $array-inline-9
(seq
(seq
(ap "parsed worker definition" $array-inline-9)
(ap worker_definition $array-inline-9)
)
(canon %init_peer_id% $array-inline-9 #array-inline-9-0)
)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-7)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-9-0] str-3)
)
(par
(call %init_peer_id% (spell_id-7 "list_push_string") ["logs" str-3])
(seq
(new $array-inline-10
(seq
(seq
(ap spell_id-7 $array-inline-10)
(ap #array-inline-9-0 $array-inline-10)
)
(canon %init_peer_id% $array-inline-10 #array-inline-10-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-10-0])
)
)
)
(fold worker_definition.$.services! s-0
(seq
(new $hashes
(xor
(seq
(seq
(seq
(seq
(fold s-0.$.modules! m-0
(seq
(seq
(seq
(seq
(seq
(call %init_peer_id% ("aqua-ipfs" "get_from") [m-0.$.wasm! ipfs] get_wasm)
(call %init_peer_id% ("aqua-ipfs" "cat_from") [m-0.$.config! ipfs] json_cfg)
)
(call %init_peer_id% ("json" "parse") [json_cfg.$.contents!] cfg)
)
(call %init_peer_id% ("dist" "add_module_from_vault") [get_wasm.$.path! cfg] hash)
)
(call %init_peer_id% ("op" "concat_strings") ["hash:" hash] $hashes)
)
(next m-0)
)
)
(par
(canon %init_peer_id% $hashes #hashes_canon)
(null)
)
)
(call %init_peer_id% ("dist" "make_blueprint") [s-0.$.name! #hashes_canon] blueprint)
)
(call %init_peer_id% ("dist" "add_blueprint") [blueprint] blueprint_id)
)
(xor
(seq
(new $result
(seq
(seq
(seq
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-8)
(call %init_peer_id% (spell_id-8 "exists") [s-0.$.name!] deployed)
)
(xor
(match deployed.$.flag! true
(xor
(seq
(seq
(call %init_peer_id% ("op" "concat_strings") ["blueprint:" s-0.$.name!] concat_strings)
(call %init_peer_id% (spell_id-8 "get_string") [concat_strings] stored_blueprint)
)
(xor
(match stored_blueprint.$.str! blueprint_id
(ap true $result)
)
(ap false $result)
)
)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 2])
)
)
(ap false $result)
)
)
(new $result_test
(seq
(seq
(seq
(call %init_peer_id% ("math" "add") [0 1] result_incr)
(fold $result s
(seq
(seq
(ap s $result_test)
(canon %init_peer_id% $result_test #result_iter_canon)
)
(xor
(match #result_iter_canon.length result_incr
(null)
)
(next s)
)
)
(never)
)
)
(canon %init_peer_id% $result_test #result_result_canon)
)
(ap #result_result_canon result_gate)
)
)
)
)
(match result_gate.$.[0]! false
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
(call %init_peer_id% (spell_id-6 "set_string") ["status" "install in progress"])
(call %init_peer_id% ("srv" "create") [blueprint_id] service_id)
)
(new $array-inline-11
(seq
(seq
(seq
(ap "Created service" $array-inline-11)
(ap s-0.$.name! $array-inline-11)
)
(ap service_id $array-inline-11)
)
(canon %init_peer_id% $array-inline-11 #array-inline-11-0)
)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-9)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-11-0] str-4)
)
(par
(call %init_peer_id% (spell_id-9 "list_push_string") ["logs" str-4])
(seq
(new $array-inline-12
(seq
(seq
(ap spell_id-9 $array-inline-12)
(ap #array-inline-11-0 $array-inline-12)
)
(canon %init_peer_id% $array-inline-12 #array-inline-12-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-12-0])
)
)
)
(xor
(call %init_peer_id% ("srv" "add_alias") [s-0.$.name! service_id])
(seq
(seq
(seq
(new $array-inline-13
(seq
(seq
(seq
(seq
(ap "Error creating alias for deployed service" $array-inline-13)
(ap s-0.$.name! $array-inline-13)
)
(ap service_id $array-inline-13)
)
(ap %last_error% $array-inline-13)
)
(canon %init_peer_id% $array-inline-13 #array-inline-13-0)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-10)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-13-0] str-5)
)
(par
(call %init_peer_id% (spell_id-10 "list_push_string") ["logs" str-5])
(seq
(new $array-inline-14
(seq
(seq
(ap spell_id-10 $array-inline-14)
(ap #array-inline-13-0 $array-inline-14)
)
(canon %init_peer_id% $array-inline-14 #array-inline-14-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-14-0])
)
)
)
)
)
(call %init_peer_id% (spell_id-6 "set_string") [s-0.$.name! service_id])
)
(call %init_peer_id% ("op" "concat_strings") ["blueprint:" s-0.$.name!] concat_strings-0)
)
(call %init_peer_id% (spell_id-6 "set_string") [concat_strings-0 blueprint_id])
)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 3])
)
)
)
(seq
(seq
(seq
(new $array-inline-15
(seq
(seq
(ap s-0.$.name! $array-inline-15)
(ap "is already deployed, doing nothing" $array-inline-15)
)
(canon %init_peer_id% $array-inline-15 #array-inline-15-0)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-11)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-15-0] str-6)
)
(par
(call %init_peer_id% (spell_id-11 "list_push_string") ["logs" str-6])
(seq
(new $array-inline-16
(seq
(seq
(ap spell_id-11 $array-inline-16)
(ap #array-inline-15-0 $array-inline-16)
)
(canon %init_peer_id% $array-inline-16 #array-inline-16-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-16-0])
)
)
)
)
)
(seq
(seq
(seq
(new $array-inline-17
(seq
(seq
(seq
(ap "Error deploying service" $array-inline-17)
(ap s-0.$.name! $array-inline-17)
)
(ap %last_error% $array-inline-17)
)
(canon %init_peer_id% $array-inline-17 #array-inline-17-0)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-12)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-17-0] str-7)
)
(par
(call %init_peer_id% (spell_id-12 "list_push_string") ["logs" str-7])
(seq
(new $array-inline-18
(seq
(seq
(ap spell_id-12 $array-inline-18)
(ap #array-inline-17-0 $array-inline-18)
)
(canon %init_peer_id% $array-inline-18 #array-inline-18-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-18-0])
)
)
)
)
)
(next s-0)
)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-13)
)
(call %init_peer_id% ("debug" "stringify") ["Installation finished"] str-8)
)
(par
(call %init_peer_id% (spell_id-13 "list_push_string") ["logs" str-8])
(seq
(new $array-inline-19
(seq
(seq
(ap spell_id-13 $array-inline-19)
(ap "Installation finished" $array-inline-19)
)
(canon %init_peer_id% $array-inline-19 #array-inline-19-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-19-0])
)
)
)
(call %init_peer_id% (spell_id-6 "set_string") ["status" "install finished"])
)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 4])
)
)
(seq
(seq
(seq
(seq
(new $array-inline-20
(seq
(seq
(seq
(seq
(ap "Error downloading worker definition from IPFS" $array-inline-20)
(ap ipfs $array-inline-20)
)
(ap worker_def_cid $array-inline-20)
)
(ap json $array-inline-20)
)
(canon %init_peer_id% $array-inline-20 #array-inline-20-0)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-14)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-20-0] str-9)
)
(par
(call %init_peer_id% (spell_id-14 "list_push_string") ["logs" str-9])
(seq
(new $array-inline-21
(seq
(seq
(ap spell_id-14 $array-inline-21)
(ap #array-inline-20-0 $array-inline-21)
)
(canon %init_peer_id% $array-inline-21 #array-inline-21-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-21-0])
)
)
)
(call %init_peer_id% (spell_id-6 "set_string") ["status" "install error"])
)
)
)
(seq
(seq
(seq
(seq
(new $array-inline-22
(seq
(seq
(ap "Error installing worker" $array-inline-22)
(ap %last_error% $array-inline-22)
)
(canon %init_peer_id% $array-inline-22 #array-inline-22-0)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-15)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-22-0] str-10)
)
(par
(call %init_peer_id% (spell_id-15 "list_push_string") ["logs" str-10])
(seq
(new $array-inline-23
(seq
(seq
(ap spell_id-15 $array-inline-23)
(ap #array-inline-22-0 $array-inline-23)
)
(canon %init_peer_id% $array-inline-23 #array-inline-23-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-23-0])
)
)
)
(call %init_peer_id% (spell_id-6 "set_string") ["status" "install error"])
)
)
)
(call %init_peer_id% (spell_id-1 "get_string") ["status"] status)
)
(new $array-inline-24
(seq
(seq
(ap "Worker installation finished with status" $array-inline-24)
(ap status.$.str! $array-inline-24)
)
(canon %init_peer_id% $array-inline-24 #array-inline-24-0)
)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-16)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-24-0] str-11)
)
(par
(call %init_peer_id% (spell_id-16 "list_push_string") ["logs" str-11])
(seq
(new $array-inline-25
(seq
(seq
(ap spell_id-16 $array-inline-25)
(ap #array-inline-24-0 $array-inline-25)
)
(canon %init_peer_id% $array-inline-25 #array-inline-25-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-25-0])
)
)
)
(xor
(match status.$.str! "install finished"
(xor
(seq
(seq
(seq
(seq
(seq
(new $successful
(new $subnetwork_id
(seq
(seq
(seq
(call %init_peer_id% ("peer" "timestamp_sec") [] t)
(xor
(seq
(seq
(seq
(seq
(call -relay- ("insecure_sig" "get_peer_id") [] peer_id)
(new $option-inline
(seq
(xor
(ap peer_id $option-inline)
(null)
)
(canon -relay- $option-inline #option-inline-0)
)
)
)
(call -relay- ("registry" "get_key_bytes") [deal_id #option-inline-0 t [] ""] bytes)
)
(call -relay- ("insecure_sig" "sign") [bytes] result-0)
)
(xor
(match result-0.$.success! false
(ap result-0.$.error.[0]! $error)
)
(seq
(seq
(seq
(seq
(seq
(seq
(ap result-0.$.signature! result-0_flat)
(call -relay- ("registry" "get_key_id") [deal_id peer_id] id)
)
(call -relay- ("op" "string_to_b58") [id] k)
)
(call -relay- ("kad" "neighborhood") [k [] []] nodes)
)
(xor
(par
(fold nodes n-0
(par
(seq
(xor
(xor
(seq
(seq
(seq
(seq
(call n-0 ("peer" "timestamp_sec") [] t-0)
(call n-0 ("trust-graph" "get_weight") [peer_id t-0] weight)
)
(new $option-inline-1
(seq
(xor
(ap peer_id $option-inline-1)
(null)
)
(canon n-0 $option-inline-1 #option-inline-1-0)
)
)
)
(call n-0 ("registry" "register_key") [deal_id #option-inline-1-0 t [] "" result-0_flat.$.[0]! weight t-0] result-1)
)
(xor
(match result-1.$.success! true
(ap true $successful)
)
(ap result-1.$.error! $error)
)
)
(call n-0 ("op" "noop") [])
)
(seq
(call -relay- ("op" "noop") [])
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 5])
)
)
(call -relay- ("op" "noop") [])
)
(next n-0)
)
(never)
)
(null)
)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 6])
)
)
(new $status-0
(new $result-2
(seq
(seq
(seq
(par
(seq
(seq
(seq
(call -relay- ("math" "sub") [1 1] sub)
(new $successful_test
(seq
(seq
(seq
(call -relay- ("math" "add") [sub 1] successful_incr)
(fold $successful s
(seq
(seq
(ap s $successful_test)
(canon -relay- $successful_test #successful_iter_canon)
)
(xor
(match #successful_iter_canon.length successful_incr
(null)
)
(next s)
)
)
(never)
)
)
(canon -relay- $successful_test #successful_result_canon)
)
(ap #successful_result_canon successful_gate)
)
)
)
(call -relay- ("math" "sub") [1 1] sub-0)
)
(ap "ok" $status-0)
)
(call -relay- ("peer" "timeout") [6000 "timeout"] $status-0)
)
(new $status-0_test
(seq
(seq
(seq
(call -relay- ("math" "add") [0 1] status-0_incr)
(fold $status-0 s
(seq
(seq
(ap s $status-0_test)
(canon -relay- $status-0_test #status-0_iter_canon)
)
(xor
(match #status-0_iter_canon.length status-0_incr
(null)
)
(next s)
)
)
(never)
)
)
(canon -relay- $status-0_test #status-0_result_canon)
)
(ap #status-0_result_canon status-0_gate)
)
)
)
(xor
(match status-0_gate.$.[0]! "ok"
(ap true $result-2)
)
(ap false $result-2)
)
)
(new $result-2_test
(seq
(seq
(seq
(call -relay- ("math" "add") [0 1] result-2_incr)
(fold $result-2 s
(seq
(seq
(ap s $result-2_test)
(canon -relay- $result-2_test #result-2_iter_canon)
)
(xor
(match #result-2_iter_canon.length result-2_incr
(null)
)
(next s)
)
)
(never)
)
)
(canon -relay- $result-2_test #result-2_result_canon)
)
(ap #result-2_result_canon result-2_gate)
)
)
)
)
)
)
(xor
(match result-2_gate.$.[0]! false
(ap "key wasn't created: timeout exceeded" $error)
)
(ap id $subnetwork_id)
)
)
)
)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 7])
)
)
(canon %init_peer_id% $subnetwork_id #-subnetwork_id-fix-0)
)
(ap #-subnetwork_id-fix-0 -subnetwork_id-flat-0)
)
)
)
(new $array-inline-26
(seq
(seq
(ap "Subnet created" $array-inline-26)
(ap -subnetwork_id-flat-0 $array-inline-26)
)
(canon %init_peer_id% $array-inline-26 #array-inline-26-0)
)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-17)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-26-0] str-12)
)
(par
(call %init_peer_id% (spell_id-17 "list_push_string") ["logs" str-12])
(seq
(new $array-inline-27
(seq
(seq
(ap spell_id-17 $array-inline-27)
(ap #array-inline-26-0 $array-inline-27)
)
(canon %init_peer_id% $array-inline-27 #array-inline-27-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-27-0])
)
)
)
(xor
(mismatch -subnetwork_id-flat-0 []
(xor
(seq
(xor
(new $successful-0
(new $error_get
(new $success
(seq
(seq
(seq
(new $error-1
(new $result-3
(seq
(seq
(seq
(seq
(seq
(seq
(seq
(seq
(call -relay- ("peer" "timestamp_sec") [] t-2)
(new $option-inline-2
(seq
(xor
(ap -relay- $option-inline-2)
(null)
)
(canon -relay- $option-inline-2 #option-inline-2-0)
)
)
)
(call -relay- ("registry" "get_record_metadata_bytes") [-subnetwork_id-flat-0.$.[0]! %init_peer_id% t-2 "" %init_peer_id% #option-inline-2-0 [] []] bytes-0)
)
(xor
(call %init_peer_id% ("sig" "sign") [bytes-0] sig_result-0)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 8])
)
)
(xor
(match sig_result-0.$.success! true
(xor
(call -relay- ("registry" "create_record_metadata") [-subnetwork_id-flat-0.$.[0]! %init_peer_id% t-2 "" %init_peer_id% #option-inline-2-0 [] [] sig_result-0.$.signature.[0]!] $result-3)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 9])
)
)
(ap sig_result-0.$.error.[0]! $error-1)
)
)
(canon -relay- $result-3 #-result-fix-0)
)
(ap #-result-fix-0 -result-flat-0)
)
(canon -relay- $error-1 #-error-fix-1)
)
(ap #-error-fix-1 -error-flat-1)
)
)
)
(xor
(match -result-flat-0 []
(seq
(ap false $success)
(ap -error-flat-1.$.[0]! $error-0)
)
)
(seq
(seq
(call -relay- ("peer" "timestamp_sec") [] t-1)
(new $signature
(seq
(seq
(call -relay- ("registry" "get_record_bytes") [-result-flat-0.$.[0]! t-1] bytes-1)
(xor
(call %init_peer_id% ("sig" "sign") [bytes-1] $signature)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 10])
)
)
(new $signature_test
(seq
(seq
(seq
(call -relay- ("math" "add") [0 1] signature_incr)
(fold $signature s
(seq
(seq
(ap s $signature_test)
(canon -relay- $signature_test #signature_iter_canon)
)
(xor
(match #signature_iter_canon.length signature_incr
(null)
)
(next s)
)
)
(never)
)
)
(canon -relay- $signature_test #signature_result_canon)
)
(ap #signature_result_canon signature_gate)
)
)
)
)
)
(xor
(match signature_gate.$.[0].success! false
(seq
(ap signature_gate.$.[0].error.[0]! $error-0)
(ap false $success)
)
)
(seq
(new $resources
(new $successful-1
(new $result-4
(seq
(seq
(seq
(seq
(seq
(seq
(call -relay- ("op" "string_to_b58") [-subnetwork_id-flat-0.$.[0]!] k-0)
(call -relay- ("kad" "neighborhood") [k-0 [] []] nodes-2)
)
(par
(fold nodes-2 n-2-0
(par
(seq
(xor
(xor
(seq
(call n-2-0 ("registry" "get_key_metadata") [-subnetwork_id-flat-0.$.[0]!] get_result)
(xor
(match get_result.$.success! true
(seq
(ap get_result.$.key! $resources)
(ap true $successful-1)
)
)
(seq
(call n-2-0 ("op" "concat_strings") [get_result.$.error! " on "] e)
(call n-2-0 ("op" "concat_strings") [e n-2-0] $error-2)
)
)
)
(call n-2-0 ("op" "noop") [])
)
(seq
(call -relay- ("op" "noop") [])
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 11])
)
)
(call -relay- ("op" "noop") [])
)
(next n-2-0)
)
(never)
)
(null)
)
)
(new $status-1
(new $result-5
(seq
(seq
(seq
(par
(seq
(seq
(seq
(call -relay- ("math" "sub") [1 1] sub-1)
(new $successful-1_test
(seq
(seq
(seq
(call -relay- ("math" "add") [sub-1 1] successful-1_incr)
(fold $successful-1 s
(seq
(seq
(ap s $successful-1_test)
(canon -relay- $successful-1_test #successful-1_iter_canon)
)
(xor
(match #successful-1_iter_canon.length successful-1_incr
(null)
)
(next s)
)
)
(never)
)
)
(canon -relay- $successful-1_test #successful-1_result_canon)
)
(ap #successful-1_result_canon successful-1_gate)
)
)
)
(call -relay- ("math" "sub") [1 1] sub-2)
)
(ap "ok" $status-1)
)
(call -relay- ("peer" "timeout") [6000 "timeout"] $status-1)
)
(new $status-1_test
(seq
(seq
(seq
(call -relay- ("math" "add") [0 1] status-1_incr)
(fold $status-1 s
(seq
(seq
(ap s $status-1_test)
(canon -relay- $status-1_test #status-1_iter_canon)
)
(xor
(match #status-1_iter_canon.length status-1_incr
(null)
)
(next s)
)
)
(never)
)
)
(canon -relay- $status-1_test #status-1_result_canon)
)
(ap #status-1_result_canon status-1_gate)
)
)
)
(xor
(match status-1_gate.$.[0]! "ok"
(ap true $result-5)
)
(ap false $result-5)
)
)
(new $result-5_test
(seq
(seq
(seq
(call -relay- ("math" "add") [0 1] result-5_incr)
(fold $result-5 s
(seq
(seq
(ap s $result-5_test)
(canon -relay- $result-5_test #result-5_iter_canon)
)
(xor
(match #result-5_iter_canon.length result-5_incr
(null)
)
(next s)
)
)
(never)
)
)
(canon -relay- $result-5_test #result-5_result_canon)
)
(ap #result-5_result_canon result-5_gate)
)
)
)
)
)
)
(xor
(match result-5_gate.$.[0]! false
(ap "resource not found: timeout exceeded" $error-2)
)
(seq
(seq
(canon -relay- $resources #resources_canon)
(call -relay- ("registry" "merge_keys") [#resources_canon] merge_result)
)
(xor
(match merge_result.$.success! true
(ap merge_result.$.key! $result-4)
)
(ap merge_result.$.error! $error-2)
)
)
)
)
(canon -relay- $result-4 #-result-fix-0-0)
)
(ap #-result-fix-0-0 -result-flat-0-0)
)
)
)
)
(xor
(match -result-flat-0-0 []
(xor
(seq
(seq
(canon -relay- $error-2 #error-2_canon)
(fold #error-2_canon e-0-0
(seq
(ap e-0-0 $error-0)
(next e-0-0)
)
)
)
(ap false $success)
)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 12])
)
)
(seq
(seq
(seq
(call -relay- ("peer" "timestamp_sec") [] t-3)
(call -relay- ("trust-graph" "get_weight") [-result-flat-0-0.$.[0].owner_peer_id! t-3] weight-0)
)
(call -relay- ("registry" "republish_key") [-result-flat-0-0.$.[0]! weight-0 t-3] result-6)
)
(xor
(match result-6.$.success! false
(seq
(ap result-6.$.error! $error-0)
(ap false $success)
)
)
(seq
(seq
(seq
(seq
(null)
(call -relay- ("peer" "timestamp_sec") [] t-4)
)
(call -relay- ("trust-graph" "get_weight") [-result-flat-0.$.[0].issued_by! t-4] weight-1)
)
(call -relay- ("registry" "put_record") [-result-flat-0.$.[0]! t-1 signature_gate.$.[0].signature.[0]! weight-1 t-4] result-7)
)
(xor
(match result-7.$.success! false
(seq
(ap result-7.$.error! $error-0)
(ap false $success)
)
)
(seq
(seq
(seq
(seq
(call -relay- ("op" "string_to_b58") [-subnetwork_id-flat-0.$.[0]!] k-1)
(call -relay- ("kad" "neighborhood") [k-1 [] []] nodes-3)
)
(par
(fold nodes-3 n-1-0
(par
(seq
(xor
(xor
(seq
(seq
(seq
(call n-1-0 ("peer" "timestamp_sec") [] t-5)
(call n-1-0 ("trust-graph" "get_weight") [-result-flat-0-0.$.[0].owner_peer_id! t-5] weight-2)
)
(call n-1-0 ("registry" "republish_key") [-result-flat-0-0.$.[0]! weight-2 t-5] result-8)
)
(xor
(match result-8.$.success! false
(ap result-8.$.error! $error-0)
)
(seq
(seq
(seq
(seq
(null)
(call n-1-0 ("peer" "timestamp_sec") [] t-6)
)
(call n-1-0 ("trust-graph" "get_weight") [-result-flat-0.$.[0].issued_by! t-6] weight-3)
)
(call n-1-0 ("registry" "put_record") [-result-flat-0.$.[0]! t-1 signature_gate.$.[0].signature.[0]! weight-3 t-6] result-9)
)
(xor
(match result-9.$.success! true
(ap true $successful-0)
)
(ap result-9.$.error! $error-0)
)
)
)
)
(call n-1-0 ("op" "noop") [])
)
(seq
(call -relay- ("op" "noop") [])
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 13])
)
)
(call -relay- ("op" "noop") [])
)
(next n-1-0)
)
(never)
)
(null)
)
)
(new $status-2
(new $result-10
(seq
(seq
(seq
(par
(seq
(seq
(seq
(call -relay- ("math" "sub") [1 1] sub-3)
(new $successful-0_test
(seq
(seq
(seq
(call -relay- ("math" "add") [sub-3 1] successful-0_incr)
(fold $successful-0 s
(seq
(seq
(ap s $successful-0_test)
(canon -relay- $successful-0_test #successful-0_iter_canon)
)
(xor
(match #successful-0_iter_canon.length successful-0_incr
(null)
)
(next s)
)
)
(never)
)
)
(canon -relay- $successful-0_test #successful-0_result_canon)
)
(ap #successful-0_result_canon successful-0_gate)
)
)
)
(call -relay- ("math" "sub") [1 1] sub-4)
)
(ap "ok" $status-2)
)
(call -relay- ("peer" "timeout") [6000 "timeout"] $status-2)
)
(new $status-2_test
(seq
(seq
(seq
(call -relay- ("math" "add") [0 1] status-2_incr)
(fold $status-2 s
(seq
(seq
(ap s $status-2_test)
(canon -relay- $status-2_test #status-2_iter_canon)
)
(xor
(match #status-2_iter_canon.length status-2_incr
(null)
)
(next s)
)
)
(never)
)
)
(canon -relay- $status-2_test #status-2_result_canon)
)
(ap #status-2_result_canon status-2_gate)
)
)
)
(xor
(match status-2_gate.$.[0]! "ok"
(ap true $result-10)
)
(ap false $result-10)
)
)
(new $result-10_test
(seq
(seq
(seq
(call -relay- ("math" "add") [0 1] result-10_incr)
(fold $result-10 s
(seq
(seq
(ap s $result-10_test)
(canon -relay- $result-10_test #result-10_iter_canon)
)
(xor
(match #result-10_iter_canon.length result-10_incr
(null)
)
(next s)
)
)
(never)
)
)
(canon -relay- $result-10_test #result-10_result_canon)
)
(ap #result-10_result_canon result-10_gate)
)
)
)
)
)
)
(ap result-10_gate.$.[0]! $success)
)
)
)
)
)
)
)
)
)
)
)
(new $success_test
(seq
(seq
(seq
(call -relay- ("math" "add") [0 1] success_incr)
(fold $success s
(seq
(seq
(ap s $success_test)
(canon -relay- $success_test #success_iter_canon)
)
(xor
(match #success_iter_canon.length success_incr
(null)
)
(next s)
)
)
(never)
)
)
(canon -relay- $success_test #success_result_canon)
)
(ap #success_result_canon success_gate)
)
)
)
(xor
(match success_gate.$.[0]! false
(ap "worker hasn't registered: timeout exceeded" $error-0)
)
(call -relay- ("op" "noop") [])
)
)
)
)
)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 14])
)
(xor
(match success_gate.$.[0]! true
(xor
(seq
(seq
(seq
(new $array-inline-28
(seq
(seq
(ap "worker successfully registered in registry" $array-inline-28)
(ap success_gate.$.[0]! $array-inline-28)
)
(canon %init_peer_id% $array-inline-28 #array-inline-28-0)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-18)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-28-0] str-13)
)
(par
(call %init_peer_id% (spell_id-18 "list_push_string") ["logs" str-13])
(seq
(new $array-inline-29
(seq
(seq
(ap spell_id-18 $array-inline-29)
(ap #array-inline-28-0 $array-inline-29)
)
(canon %init_peer_id% $array-inline-29 #array-inline-29-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-29-0])
)
)
)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 15])
)
)
(seq
(seq
(seq
(new $array-inline-30
(seq
(seq
(seq
(ap "error registering worker" $array-inline-30)
(canon %init_peer_id% $error-0 #push-to-stream-844)
)
(ap #push-to-stream-844 $array-inline-30)
)
(canon %init_peer_id% $array-inline-30 #array-inline-30-0)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-19)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-30-0] str-14)
)
(par
(call %init_peer_id% (spell_id-19 "list_push_string") ["logs" str-14])
(seq
(new $array-inline-31
(seq
(seq
(ap spell_id-19 $array-inline-31)
(ap #array-inline-30-0 $array-inline-31)
)
(canon %init_peer_id% $array-inline-31 #array-inline-31-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-31-0])
)
)
)
)
)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 16])
)
)
(seq
(seq
(seq
(new $array-inline-32
(seq
(seq
(seq
(ap "error creating subnetwork" $array-inline-32)
(canon %init_peer_id% $error #push-to-stream-862)
)
(ap #push-to-stream-862 $array-inline-32)
)
(canon %init_peer_id% $array-inline-32 #array-inline-32-0)
)
)
(call %init_peer_id% ("getDataSrv" "spell_id") [] spell_id-20)
)
(call %init_peer_id% ("debug" "stringify") [#array-inline-32-0] str-15)
)
(par
(call %init_peer_id% (spell_id-20 "list_push_string") ["logs" str-15])
(seq
(new $array-inline-33
(seq
(seq
(ap spell_id-20 $array-inline-33)
(ap #array-inline-32-0 $array-inline-33)
)
(canon %init_peer_id% $array-inline-33 #array-inline-33-0)
)
)
(call %init_peer_id% ("run-console" "print") [#array-inline-33-0])
)
)
)
)
)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 17])
)
)
(call %init_peer_id% ("op" "noop") [])
)
)
)
)
)
)
)
)
(call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 18])
)"#;

#[test]
fn issue__() {
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

    /*
    let current_data: InterpreterData = serde_json::from_str(CURRENT_DATA).unwrap();
    let current_data: Vec<u8> = serde_json::to_vec(&current_data).unwrap();
     */

    let mut call_results = CallResults::new();
    call_results.insert(70, <_>::default());

    let result = client_vm
        .runner
        .call(
            AIR,
            prev_data,
            "",
            "12D3KooW9xAAjbEWMq7VLV3ihLa1dmZBxYS9R4MWNuEzCCZypsw1",
            1677418607051,
            120000,
            None,
            call_results,
        )
        .unwrap();

    assert_eq!(result.ret_code, 0);
    println!("result: {:?}", result);
}
