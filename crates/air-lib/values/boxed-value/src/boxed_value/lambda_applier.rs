use crate::BoxedValue;
use air_lambda_ast::AIRLambda;

pub trait LambdaApplier {
    fn apply_lambda<'value>(
        &'value self,
        lambda: &AIRLambda<'_>,
    ) -> Result<&(dyn BoxedValue + 'value), ValueLambdaError>;
}

use thiserror::Error as ThisError;

/// Describes errors related to applying lambdas to values.
#[derive(Debug, Clone, ThisError)]
pub enum ValueLambdaError {
    #[error("value '{value}' does not contain element for idx = '{idx}'")]
    ValueNotContainSuchArrayIdx { value: String, idx: u32 },

    #[error("value '{value}' does not contain element with field name = '{field_name}'")]
    ValueNotContainSuchField { value: String, field_name: String },
}
