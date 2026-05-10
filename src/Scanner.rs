use crate::ErrorHandling;
use crate::Token;
use crate::TokenType;

pub struct Scanner {
    source: String,
    pub hadError: bool
}


impl Scanner {

    pub fn new(source: String) -> Self {
        Self { source: source, hadError: false }
    }

    pub fn scanTokens(&self) -> Vec<Token::Token> {
        return Vec::from([Token::Token{
            tType: TokenType::TokenType::BANG, 
            lexeme: String::from("test"),
            literal: String::from("1"),
            line: 100
        }]);
    }

}
