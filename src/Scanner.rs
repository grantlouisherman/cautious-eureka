
pub struct Scanner {
    source: String
}

pub struct Token {
    tmp_value: String
}

impl Scanner {

    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn scanTokens(&self) -> Vec<Token> {
        return Vec::from([Token{tmp_value: String::from("test")}]);
    }

}
