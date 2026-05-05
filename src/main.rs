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
    r";",
    r"0|[1-9][0-9]*",
    r"=",
    r""
];

fn main() {
    let code = r#"
        int x = 100;
        int y = 2;
        string hello = "hello";
    "#;
    let split:Vec<_> = code.split("\n").collect();
    
    let scanner = ScannerBuilder::new()
        .add_patterns(PATTERNS)
        .build()
        .expect("ScannerBuilder error");
    let mut tokens = vec![];
    for(_, line) in split.iter().enumerate() {
        let find_iter = scanner.find_iter(line);
        for _ma in find_iter {
            tokens.push(_ma);
        }
    }

    for token in tokens {
        println!("{:?}", token);

    }

}
