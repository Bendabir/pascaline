use thiserror::Error;

// Errors for the app
#[derive(Debug, Error)]
pub enum PascalineError<'a> {
    #[error("Failed to create operator from symbol : '{0}'")]
    OperatorSymbolError(&'a str),
    #[error("Not enough values to apply operator '{op}' (expected : {expected}, found : {found})")]
    ArityError {
        op: &'a str,
        expected: usize,
        found: usize
    },
    #[error("Unexpected type")]
    TypeError,
    #[error("Division by zero is impossible")]
    ZeroDivisionError,
    #[error("Computation stack is full")]
    FullStackError,
    #[error("Computation stack is empty")]
    EmptyStackError,
    #[error("Operation is not implemented yet")]
    NotImplementedError,
    #[error("No last operator to apply")]
    NoLastOperatorError,
    // Just a work around for borrowing issues
    // Using a flag to delay some computation in the code
    #[error("")]
    LastOperatorFlag
}
