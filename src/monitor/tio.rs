use std::io::Write;

pub fn out(data: &str){
    std::io::stdout().write(data.as_bytes()).unwrap();
    std::io::stdout().flush();
}

pub fn readln() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.pop();
    line.pop();
    return line;
}