use crate::scanner::Scanner;
use crate::scanner::TokenType;

fn compile(source: String) {
    let mut scanner = Scanner::from(source);
    let mut line = 0;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            line = token.line;
            print!("{line} ");
        } else {
            print!(" | ");
        }
        println!("{token:?}");

        match token.token_type {
            TokenType::EOF => break,
            _ => break,
        }
    }
}
