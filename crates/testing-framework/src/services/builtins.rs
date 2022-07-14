use super::{FunctionOutcome, Service};

/// Actually the service is stateful: it access current time, Neighbourhood,
pub struct Builtins {}

impl Service for Builtins {
    fn call(
        &self,
        service_id: &str,
        method: &str,
        args: &[super::JValue],
    ) -> FunctionOutcome {
        match method {

            _ => FunctionOutcome::NotDefined,
        }
    }
}
