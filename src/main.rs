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
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub scanner);

fn main() {
    assert!(scanner::TermParser::new().parse("22").is_ok());
}
