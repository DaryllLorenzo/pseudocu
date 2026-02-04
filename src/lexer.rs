use std::iter::{self, from_fn};

pub enum Token {
    Number(i64),
    Plus,
    Dash,
    Star,
    EOF
}

#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
}

impl SyntaxError {
    fn new(message: String) -> Self {
        SyntaxError {
            message
        }
    }
}


pub fn tokenizer(input: String) -> Result<Vec<Token>, SyntaxError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(ch) = iter.next() {
        match ch {
            ch if ch.is_whitespace() => continue,
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Dash),
            '*' => tokens.push(Token::Star),
            '1'..='9' => {
                let n: i64 = iter::once(ch)
                    .chain(from_fn(|| iter.by_ref().next_if(|s| s.is_ascii_digit())))
                    .collect::<String>()
                    .parse()
                    .unwrap();

                tokens.push(Token::Number(n));
            }
            _ => return Err(SyntaxError::new(format!("unrecognized character {}", ch))),
        }
    }

    tokens.push(Token::EOF);
    Ok(tokens)
}

