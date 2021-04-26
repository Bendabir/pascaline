use thiserror::Error;

// Errors for the app
#[derive(Debug, Error)]
pub enum PascalineError<'a> {
    #[error("Failed to create operator from symbol : '{0}'")]
    OperatorSymbolError(&'a str),
    #[error("Wrong arity for operator '{op}' (expected : {expected}, got : {got})")]
    ArityError {
        op: &'a str,
        expected: u8,
        got: u8
    },
    #[error("Unexpected type")]
    TypeError,
    #[error("Division by zero is impossible")]
    ZeroDivisionError
}
