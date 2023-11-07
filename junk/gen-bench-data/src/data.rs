use std::collections::HashMap;

#[derive(Debug, Default)]
#[allow(dead_code)]
pub(crate) struct Data {
    pub(crate) air: String,
    pub(crate) prev_data: Vec<u8>,
    pub(crate) cur_data: Vec<u8>,
    pub(crate) params_json: HashMap<String, String>,
    pub(crate) call_results: Option<serde_json::Value>,
    pub(crate) keypair: String,
}
