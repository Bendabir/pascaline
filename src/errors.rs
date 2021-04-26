use thiserror::Error;

// Errors for the app
#[derive(Debug, Error)]
pub enum PascalineError {
    #[error("Failed to create operator from symbol : '{0}'")]
    OperatorSymbolError(String)
}
