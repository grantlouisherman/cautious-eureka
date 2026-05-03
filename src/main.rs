/*
 * expression -> variable_decl | equation | literal
 * literal -> NUMBER | STRING
 * variable_decl -> var_type literal "=" literal
 * var_type -> int | str 
 * equation -> binary
 *  binary -> literal operator literal
 *  operator -> "+"
 *
 * */


#[derive(Debug, Clone)] 
enum LineType {
    VarDecl,
    Equation
}
#[derive(Debug, Clone)] 
enum TokenType {
  VarType(String),
  VarName(String),
  VarValueNum(String),
  VarValueString(String),
  EqAdd(String, String),
  EqSub(String, String),
  LiteralNum(String),
  LiteralString(String),
  Operator(String),

}
#[derive(Debug, Clone)] 
struct Token {
    line_num: usize,
    line_type: LineType,
    tokens: Vec<TokenType>
}

const RESERVED_NAMES: [&str; 2]= ["int", "string"];

fn compile(code_string: String) -> Vec<Token> {
    let code: Vec<_> = code_string.split('\n').collect();
    let mut tokens: Vec<Token> = vec![];
    for (idx, value) in code.iter().enumerate() {
        let line:Vec<_> = value.split(" ").collect();
        if line[0] == "int" {
            tokens.push(Token{
                line_num: idx,
                line_type: LineType::VarDecl,
                tokens: [
                TokenType::VarType(line[0].to_string()), 
                TokenType::VarName(line[1].to_string()), 
                TokenType::VarValueNum(line[3].to_string().replace(";", ""))].to_vec()
            }) 
        }
        if value.contains("+") {
            tokens.push(Token{
                line_num: idx,
                line_type: LineType::Equation,
                tokens: [
                TokenType::VarType(line[0].to_string()), 
                TokenType::VarName(line[1].to_string()), 
                TokenType::VarValueNum(line[3].to_string().replace(";", ""))].to_vec()
            });
            }
    }
    tokens
}
fn main() {
  let tokens = compile(String::from("int x = 5;\nint y = 4;\n1+2;"));
  for token in tokens {
      println!("{:?}", token);
  }
}
