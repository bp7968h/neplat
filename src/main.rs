use std::{env, fs::File, io::Read, process};

use neplat::{Interpreter, Lexer, Parser};

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
                    // Lexing
                    let mut lexer = Lexer::new(contents.as_bytes());
                    let tokens = lexer.tokenize();

                    if !lexer.get_errors().is_empty() {
                        eprintln!("Lex Errors encountered: ");
                        for error in lexer.get_errors() {
                            println!("\t{}", error);
                        }
                        process::exit(1);
                    }

                    // Parsing
                    let mut parser = Parser::new(&tokens);
                    let statements = parser.parse();

                    if !parser.get_errors().is_empty() {
                        eprintln!("Parse Errors encountered: ");
                            for error in parser.get_errors() {
                                eprintln!("\t{}", error);
                            }
                            process::exit(1);
                    }

                    let mut interpreter = Interpreter::new();
                    if let Err(errors) = interpreter.interpret(&statements) {
                        eprintln!("Runtime Errors encountered: ");
                        for error in errors {
                            eprintln!("\t{}", error);
                        }
                    }
                    
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
