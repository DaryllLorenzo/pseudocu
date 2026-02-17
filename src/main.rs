use std::env;
use std::fs;
use pseudocu::lexer;
use pseudocu::parser;
use pseudocu::interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let input = if args.len() > 1 {
        match fs::read_to_string(&args[1]) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", args[1], e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Usage: pseudocu <file.pc>");
        std::process::exit(1);
    };

    let tokens = match lexer::tokenize(input) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {:?}", e);
            std::process::exit(1);
        }
    };

    let mut parser = parser::Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parser error: {}", e);
            std::process::exit(1);
        }
    };

    let mut interpreter = interpreter::Interpreter::new();
    if let Err(e) = interpreter.run(program) {
        eprintln!("Runtime error: {}", e);
        std::process::exit(1);
    }

    interpreter.print_variables();
}