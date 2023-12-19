use std::io::{stdin, stdout, Write};

pub mod cursor;
pub mod interpreter;
pub mod lexer;
pub mod parser;

fn main() {
    let mut program = interpreter::ProgramContext::new();
    let mut line = String::new();

    loop {
        print!(">> ");

        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();

        let source = line.trim();

        if source.is_empty() {
            continue;
        } else if source == "exit" {
            break;
        }

        program.interpret(source);

        line.clear();
    }
}
