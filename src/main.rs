use std::{
    io::{self},
    process::exit,
};

mod tokenizer;


fn main() {
    loop {
        let mut buffer: String = String::new();
        let stdin = io::stdin();

        match stdin.read_line(&mut buffer) {
            Ok(exp) => {
                if size_of_val(&exp) == 0 {
                    buffer.clear();
                    continue;
                }
                if buffer.contains("exit") {
                    buffer.clear();
                    println!("Exiting....");
                    exit(0)
                }
            }
            Err(error) => {
                println!("{error}");
            }
        }
    }
}
