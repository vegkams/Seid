use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum Error {
    #[error("Usage: seid <filename> <[params]>")]
    InputError(String),
    #[error("REPL: {0}")]
    Repl(String),
    #[error("[line {0}] Error{1}: {2}")]
    SyntaxError(String, String, String),
    #[error("Anyhow: {0}")]
    Anyhow(String),
}

impl std::convert::From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::Anyhow(e.to_string())
    }
}