#[derive(Debug, Clone)] 
enum LineType {
    VarDecl,
    Expr
}
#[derive(Debug, Clone)] 
enum TokenType {
  VarType(String),
  VarName(String),
  VarValueNum(String),
  VarValueString(String),
  ExprAdd(String, String),
  ExprSub(String, String),

}
#[derive(Debug, Clone)] 
struct Token {
    line_num: usize,
    line_type: LineType,
    tokens: [TokenType; 3]
}

const RESERVED_NAMES: [&str; 2]= ["int", "string"];

fn compile(code_string: String) -> Vec<Token> {
    let code: Vec<_> = code_string.split('\n').collect();
    let mut tokens: Vec<Token> = vec![];
    for (idx, value) in code.iter().enumerate() {
        let line:Vec<_> = value.split(" ").collect();
        println!("{:?}", line);
        if line[0] == "int" {
            tokens.push(Token{
                line_num: idx,
                line_type: LineType::VarDecl,
                tokens: [
                TokenType::VarType(line[0].to_string()), 
                TokenType::VarName(line[1].to_string()), 
                TokenType::VarValueNum(line[3].to_string())]
            }) 
        }
        
    }
    tokens
}
fn main() {
  let tokens = compile(String::from("int x = 5;\nint y = 4;"));
  for token in tokens {
      println!("{:?}", token);
  }
}
