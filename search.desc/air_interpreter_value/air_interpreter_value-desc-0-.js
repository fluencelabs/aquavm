searchState.loadedDescShard("air_interpreter_value", 0, "Represents a JSON array.\nRepresents a JSON boolean.\nRepresents any valid JSON value with a cheap to clone …\nRepresents a JSON null value.\nRepresents a JSON number, whether integer or floating …\nRepresents a JSON object.\nRepresents a JSON string.\nIf the <code>JValue</code> is an Array, returns the associated vector. …\nIf the <code>JValue</code> is a Boolean, returns the associated bool. …\nIf the <code>JValue</code> is a number, represent it as f64 if …\nIf the <code>JValue</code> is an integer, represent it as i64 if …\nIf the <code>JValue</code> is a Null, returns (). Returns None …\nIf the <code>JValue</code> is a Number, returns the associated <code>Number</code>. …\nIf the <code>JValue</code> is an Object, returns the associated Map. …\nIf the <code>JValue</code> is a string, returns the associated str. …\nIf the <code>JValue</code> is an integer, represent it as u64 if …\nDisplay a JSON value as a string.\nReturns the argument unchanged.\nConvert map (with string keys) to <code>JValue::Object</code>.\nConvert <code>serde_json::Number</code> to <code>JValue::Number</code>.\nConvert <code>String</code> to <code>JValue::String</code>.\nConvert map (with string keys) to <code>JValue::Object</code>.\nConvert boolean to <code>JValue::Bool</code>.\nConvert a slice to <code>JValue::Array</code>.\nConvert copy-on-write string to <code>JValue::String</code>.\nConvert 64-bit floating point number to <code>JValue::Number</code>, or …\nConvert a <code>Vec</code> to <code>JValue::Array</code>.\nConvert 32-bit floating point number to <code>JValue::Number</code>, or …\nConvert string slice to <code>JValue::String</code>.\nConvert <code>()</code> to <code>JValue::Null</code>.\nConvert <code>JsonString</code> to <code>JValue::String</code>.\nCreate a <code>JValue::Object</code> by collecting an iterator of …\nCreate a <code>JValue::Array</code> by collecting an iterator of array …\nIndex into a JSON array or map. A string index can be used …\nIndex into a <code>air_interpreter_value::JValue</code> using the …\nCalls <code>U::from(self)</code>.\nReturns true if the <code>JValue</code> is an Array. Returns false …\nReturns true if the <code>JValue</code> is a Boolean. Returns false …\nReturns true if the <code>JValue</code> is a number that can be …\nReturns true if the <code>JValue</code> is an integer between <code>i64::MIN</code> …\nReturns true if the <code>JValue</code> is a Null. Returns false …\nReturns true if the <code>JValue</code> is a Number. Returns false …\nReturns true if the <code>JValue</code> is an Object. Returns false …\nReturns true if the <code>JValue</code> is a String. Returns false …\nReturns true if the <code>JValue</code> is an integer between zero and …\nLooks up a value by a JSON Pointer.\nTakes the value out of the <code>JValue</code>, leaving a <code>Null</code> in its …")