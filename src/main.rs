#![allow(unused_variables)]
mod parser;
mod tokenizer;

use rustyline::{error::ReadlineError, DefaultEditor};

/// Displays help information for the calculator.
fn display_help() {
    println!("Type a mathematical expression to tokenize it.");
    println!("Type 'help' or '\\h' to display this.");
    println!("Type 'exit' or '\\e' to quit.");
    println!("You can enter expressions using numbers, variables (like x), operators (+, -, *, /, ^), and parentheses.");
    println!("Examples:");
    println!("  2*x + 3");
    println!("  (x^2+2*x+1)/(x+1) ");
}

fn main() -> rustyline::Result<()> {
    let mut read_line = DefaultEditor::new()?;
    read_line.append_history("history.txt")?;
    println!("\n====Symbolic Derivative Calculator====\n");
    loop {
        let line = match read_line.readline("Expr> ") {
            Ok(line) => {
                if line.trim() == "exit" || line.trim() == "\\e" {
                    println!("Exiting...");
                    println!("Bye!!");
                    break Ok(());
                }
                if line.trim() == "help" || line.trim() == "\\h" {
                    display_help();
                    continue;
                }
                let mut tokenizer = tokenizer::Tokenizer::new(&line);
                let tokens = tokenizer.tokenize().unwrap();
                let mut parser = parser::Parser::new(tokens.clone());
                match parser.parse() {
                    Some(ast) => println!("Parsed AST: {ast:?}"),
                    None => println!("Parse error: "),
                }
                println!("{tokens:?}");
            }

            Err(e) => {
                match e {
                    ReadlineError::Interrupted => println!("CTRL-C detected!!"),
                    ReadlineError::Eof => println!("CTRL-D detected!!"),
                    other => println!("Error: {other:?}"),
                }
                break Ok(());
            }
        };
    }
}
