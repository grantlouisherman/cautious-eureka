use crate::TokenType::TokenType;


#[derive(Debug)]
pub struct Token {
    pub tType: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: i32
}


impl Token {
    
    pub fn new(tType: TokenType, lexeme: String, literal:String, line: i32) -> Self {
        Self {
            tType: tType,
            lexeme: lexeme,
            literal: literal,
            line:line
        }
    }

    pub fn toString(&self) -> String {
        return format!("{} {} {}", self.lexeme, self.literal, self.line);    
    }
}
