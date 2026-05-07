fn report(line: i32, _where: &str, message: &str){
    eprintln!("[line {}] Error {}: {}", line, _where, message);
}

pub fn error(line: i32, message: &str) {
    report(line, "", message);
}
