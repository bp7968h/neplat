use std::{env, fs::File, io::Read, process};

use netlat::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => run(&args[1]),
        _ => {
            eprintln!("Usage: neplat [script]");
            process::exit(1);
        }
    }
}

fn run(file_name: &str) {
    match File::open(file_name) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    let lexer = Lexer::new(contents);
                    todo!()
                },
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
