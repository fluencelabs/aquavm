(function() {var type_impls = {
"air_interpreter_interface":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-RmpSerdeMultiformat\" class=\"impl\"><a class=\"src rightside\" href=\"src/air_interpreter_sede/rmp_serde.rs.html#73\">source</a><a href=\"#impl-Clone-for-RmpSerdeMultiformat\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"air_interpreter_sede/rmp_serde/struct.RmpSerdeMultiformat.html\" title=\"struct air_interpreter_sede::rmp_serde::RmpSerdeMultiformat\">RmpSerdeMultiformat</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/air_interpreter_sede/rmp_serde.rs.html#73\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"air_interpreter_sede/rmp_serde/struct.RmpSerdeMultiformat.html\" title=\"struct air_interpreter_sede::rmp_serde::RmpSerdeMultiformat\">RmpSerdeMultiformat</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","air_interpreter_interface::call_request_parameters::CallRequestsFormat","air_interpreter_interface::call_service_result::CallResultsFormat"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-RmpSerdeMultiformat\" class=\"impl\"><a class=\"src rightside\" href=\"src/air_interpreter_sede/rmp_serde.rs.html#73\">source</a><a href=\"#impl-Default-for-RmpSerdeMultiformat\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"air_interpreter_sede/rmp_serde/struct.RmpSerdeMultiformat.html\" title=\"struct air_interpreter_sede::rmp_serde::RmpSerdeMultiformat\">RmpSerdeMultiformat</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/air_interpreter_sede/rmp_serde.rs.html#73\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; <a class=\"struct\" href=\"air_interpreter_sede/rmp_serde/struct.RmpSerdeMultiformat.html\" title=\"struct air_interpreter_sede::rmp_serde::RmpSerdeMultiformat\">RmpSerdeMultiformat</a></h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","air_interpreter_interface::call_request_parameters::CallRequestsFormat","air_interpreter_interface::call_service_result::CallResultsFormat"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Format%3CValue%3E-for-RmpSerdeMultiformat\" class=\"impl\"><a class=\"src rightside\" href=\"src/air_interpreter_sede/rmp_serde.rs.html#76-78\">source</a><a href=\"#impl-Format%3CValue%3E-for-RmpSerdeMultiformat\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;Value&gt; <a class=\"trait\" href=\"air_interpreter_sede/format/trait.Format.html\" title=\"trait air_interpreter_sede::format::Format\">Format</a>&lt;Value&gt; for <a class=\"struct\" href=\"air_interpreter_sede/rmp_serde/struct.RmpSerdeMultiformat.html\" title=\"struct air_interpreter_sede::rmp_serde::RmpSerdeMultiformat\">RmpSerdeMultiformat</a><div class=\"where\">where\n    Value: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.195/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>,</div></h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.SerializationError\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.SerializationError\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"air_interpreter_sede/format/trait.Format.html#associatedtype.SerializationError\" class=\"associatedtype\">SerializationError</a> = <a class=\"enum\" href=\"air_interpreter_sede/multiformat/enum.EncodeError.html\" title=\"enum air_interpreter_sede::multiformat::EncodeError\">EncodeError</a>&lt;Error&gt;</h4></section><section id=\"associatedtype.DeserializationError\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.DeserializationError\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"air_interpreter_sede/format/trait.Format.html#associatedtype.DeserializationError\" class=\"associatedtype\">DeserializationError</a> = <a class=\"enum\" href=\"air_interpreter_sede/multiformat/enum.DecodeError.html\" title=\"enum air_interpreter_sede::multiformat::DecodeError\">DecodeError</a>&lt;Error&gt;</h4></section><section id=\"associatedtype.WriteError\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.WriteError\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"air_interpreter_sede/format/trait.Format.html#associatedtype.WriteError\" class=\"associatedtype\">WriteError</a> = <a class=\"enum\" href=\"air_interpreter_sede/multiformat/enum.EncodeError.html\" title=\"enum air_interpreter_sede::multiformat::EncodeError\">EncodeError</a>&lt;Error&gt;</h4></section><section id=\"method.to_vec\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/air_interpreter_sede/rmp_serde.rs.html#85\">source</a><a href=\"#method.to_vec\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"air_interpreter_sede/format/trait.Format.html#tymethod.to_vec\" class=\"fn\">to_vec</a>(\n    &amp;self,\n    value: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Value</a>,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>&gt;, &lt;<a class=\"struct\" href=\"air_interpreter_sede/rmp_serde/struct.RmpSerdeMultiformat.html\" title=\"struct air_interpreter_sede::rmp_serde::RmpSerdeMultiformat\">RmpSerdeMultiformat</a> as <a class=\"trait\" href=\"air_interpreter_sede/format/trait.Format.html\" title=\"trait air_interpreter_sede::format::Format\">Format</a>&lt;Value&gt;&gt;::<a class=\"associatedtype\" href=\"air_interpreter_sede/format/trait.Format.html#associatedtype.SerializationError\" title=\"type air_interpreter_sede::format::Format::SerializationError\">SerializationError</a>&gt;</h4></section><section id=\"method.from_slice\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/air_interpreter_sede/rmp_serde.rs.html#90\">source</a><a href=\"#method.from_slice\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"air_interpreter_sede/format/trait.Format.html#tymethod.from_slice\" class=\"fn\">from_slice</a>(\n    &amp;self,\n    slice: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>],\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Value, &lt;<a class=\"struct\" href=\"air_interpreter_sede/rmp_serde/struct.RmpSerdeMultiformat.html\" title=\"struct air_interpreter_sede::rmp_serde::RmpSerdeMultiformat\">RmpSerdeMultiformat</a> as <a class=\"trait\" href=\"air_interpreter_sede/format/trait.Format.html\" title=\"trait air_interpreter_sede::format::Format\">Format</a>&lt;Value&gt;&gt;::<a class=\"associatedtype\" href=\"air_interpreter_sede/format/trait.Format.html#associatedtype.DeserializationError\" title=\"type air_interpreter_sede::format::Format::DeserializationError\">DeserializationError</a>&gt;</h4></section><section id=\"method.to_writer\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/air_interpreter_sede/rmp_serde.rs.html#95-99\">source</a><a href=\"#method.to_writer\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"air_interpreter_sede/format/trait.Format.html#tymethod.to_writer\" class=\"fn\">to_writer</a>&lt;W&gt;(\n    &amp;self,\n    value: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;Value</a>,\n    write: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.reference.html\">&amp;mut W</a>,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, &lt;<a class=\"struct\" href=\"air_interpreter_sede/rmp_serde/struct.RmpSerdeMultiformat.html\" title=\"struct air_interpreter_sede::rmp_serde::RmpSerdeMultiformat\">RmpSerdeMultiformat</a> as <a class=\"trait\" href=\"air_interpreter_sede/format/trait.Format.html\" title=\"trait air_interpreter_sede::format::Format\">Format</a>&lt;Value&gt;&gt;::<a class=\"associatedtype\" href=\"air_interpreter_sede/format/trait.Format.html#associatedtype.WriteError\" title=\"type air_interpreter_sede::format::Format::WriteError\">WriteError</a>&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,</div></h4></section></div></details>","Format<Value>","air_interpreter_interface::call_request_parameters::CallRequestsFormat","air_interpreter_interface::call_service_result::CallResultsFormat"],["<section id=\"impl-Copy-for-RmpSerdeMultiformat\" class=\"impl\"><a class=\"src rightside\" href=\"src/air_interpreter_sede/rmp_serde.rs.html#73\">source</a><a href=\"#impl-Copy-for-RmpSerdeMultiformat\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for <a class=\"struct\" href=\"air_interpreter_sede/rmp_serde/struct.RmpSerdeMultiformat.html\" title=\"struct air_interpreter_sede::rmp_serde::RmpSerdeMultiformat\">RmpSerdeMultiformat</a></h3></section>","Copy","air_interpreter_interface::call_request_parameters::CallRequestsFormat","air_interpreter_interface::call_service_result::CallResultsFormat"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()