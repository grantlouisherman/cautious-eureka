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

use scnr::ScannerBuilder;

static PATTERNS: &[&str] = &[
    r";",                    // Semicolon
    r"0|[1-9][0-9]*",        // Number
    r"//.*(\r\n|\r|\n)",     // Line comment
    r"/\*([^*]|\*[^/])*\*/", // Block comment
    r"[a-zA-Z_]\w*",         // Identifier
    r"=",                    // Assignment
];

const INPUT: &str = r#"
// This is a comment
int a = 10;
int b = 20;
string y = "hello";
/* This is a block comment
   that spans multiple lines */
int c = a;
"#;

fn main() {
    let scanner = ScannerBuilder::new()
        .add_patterns(PATTERNS)
        .build()
        .expect("ScannerBuilder error");
    let find_iter = scanner.find_iter(INPUT);
    for ma in find_iter {
        println!("Match: {:?}: '{}'", ma, &INPUT[ma.span().range()]);
    }
}
