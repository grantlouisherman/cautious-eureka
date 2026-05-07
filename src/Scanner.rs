use crate::ErrorHandling;

pub struct Scanner {
    source: String,
    pub hadError: bool
}
#[derive(Debug)]
pub struct Token {
    tmp_value: String
}

impl Scanner {

    pub fn new(source: String) -> Self {
        Self { source: source, hadError: false }
    }

    pub fn scanTokens(&self) -> Vec<Token> {
        return Vec::from([Token{tmp_value: String::from("test")}]);
    }

}
