use std::io::{Stdin, Stdout, Write};

use crate::{lexer::Lexer, token::TokenKind};

pub fn start(stdin: Stdin, mut stdout: Stdout) {
    loop {
        write!(stdout, ">> ").expect("should have written prompt string");
        stdout.flush().expect("should have flushed stdout");

        let mut input = String::new();

        if let Err(e) = stdin.read_line(&mut input) {
            writeln!(stdout, "Error: {e}").expect("should have written error message");
            return;
        }

        let mut lexer = Lexer::new(input.as_str());

        loop {
            let tk = lexer.next_token();

            if tk.kind == TokenKind::Eof {
                break;
            }

            writeln!(stdout, "{tk:?}").expect("token should have been written");
        }
    }
}
