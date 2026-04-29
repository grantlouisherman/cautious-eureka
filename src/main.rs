#[derive(Debug)] 
enum LineType {
    VarDecl,
    Expr
}
#[derive(Debug)] 
enum TokenType {
  VarType(String),
  VarName(String),
  VarValueNum(String),
  VarValueString(String),
}
#[derive(Debug)] 
struct Token {
    line_num: usize,
    line_type: LineType,
    tokens: Vec<TokenType>
}

const RESERVED_NAMES: [String; 3] = ["int", "string"];

fn compile(code_string: String) -> Vec<Token> {
    let code: Vec<_> = code_string.split('\n').collect();
    let mut tokens: Vec<Token> = vec![];
    for (idx, value) in code.iter().enumerate() {
        let line = value.split(" ");
        
    }
    tokens
}
fn main() {
  let tokens = compile(String::from("int x = 5;\nint y = 4;"));
  for token in tokens {
      println!("{:?}", token);
  }
}
