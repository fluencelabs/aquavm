use air::RunParameters;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AquaVMParameters {
    pub air: String,
    pub prev_data: Vec<u8>,
    pub data: Vec<u8>,
    pub params: RunParameters,
    pub call_results: Vec<u8>,
}
