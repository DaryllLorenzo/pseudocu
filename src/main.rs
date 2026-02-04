mod lexer;
use lexer::{tokenizer, Token};

fn main() {
    let input = String::from("1 + 2 * 3");
    
    match tokenizer(input) {
        Ok(tokens) => {
            for token in tokens {
                match token {
                    Token::Number(n) => println!("Number: {}", n),
                    Token::Plus => println!("Operator: +"),
                    Token::Dash => println!("Operator: -"),
                    Token::Star => println!("Operator: *"),
                    Token::EOF => println!("EOF"),
                }
            }
        }
        Err(e) => println!("Error: {}", e.message),
    }
}