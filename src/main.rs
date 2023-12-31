use std::io;

pub mod ast;
pub mod lexer;
pub mod repl;
pub mod token;

fn main() {
    println!("Hello! This is the Monkey Programming Language!");
    println!("Feel free to type the code");
    repl::start(io::stdin(), io::stdout());
}
