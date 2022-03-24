use super::Value;
use air_lambda_ast::LambdaAST;

pub trait LambdaApplier {
    fn apply_lambda<'value>(&'value self, lambda_ast: &LambdaAST<'_>) -> &(dyn Value + 'value);
}
