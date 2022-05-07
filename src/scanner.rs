use crate::Token;
use crate::Error;

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: String::from(source),
        }
    }

    pub fn scan_tokens(&self) -> Result<Vec<Token>, Error> {
        Ok(Vec::<Token>::new())
    }
}