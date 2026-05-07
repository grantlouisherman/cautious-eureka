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
use std::env;
use std::fs;
use std::io;
mod Scanner;

use Scanner::{Scanner as ScannerClass, Token};

fn runFile(path: String){
    let contents = fs::read_to_string(path).expect("Failed to read file");
    println!("{:?}", contents);
    let scanner = ScannerClass::new(contents);
    let tokens = scanner.scanTokens();
}

fn runPrompt() {
    let mut buffer = String::new();
    loop {
        println!("> ");
        io::stdin().read_line(&mut buffer).expect("IO Failed");
        if buffer.trim() == "exit" {
            break
        }
        buffer.clear();
    }
    println!("exit prompt");
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
    } else if args.len() == 2 {
        runFile(args[1].clone());
    } else {
        runPrompt();
    }

}
