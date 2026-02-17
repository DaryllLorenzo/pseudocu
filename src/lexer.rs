#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Number(i64),
    Ident(String),
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    NotEq,
    Gt,
    Lt,
    GtEq,
    LtEq,
    EOF,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
    line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
            line: 1,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        let prev_ch = self.ch;

        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;

        if prev_ch == b'\n' {
            self.line += 1;
        }
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let end = self.position;
        std::str::from_utf8(&self.input[start..end])
            .unwrap_or("")
            .to_string()
    }

    fn read_ident(&mut self) -> String {
        let start = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch.is_ascii_digit() || self.ch == b'_' {
            self.read_char();
        }
        let end = self.position;
        std::str::from_utf8(&self.input[start..end])
            .unwrap_or("")
            .to_string()
    }

    pub fn next_token(&mut self) -> Token {
        // Skip whitespace
        while matches!(self.ch, b' ' | b'\t' | b'\n' | b'\r') {
            self.read_char();
        }

        let current_line = self.line;

        match self.ch {
            b'=' => {
                self.read_char();
                if self.ch == b'=' {
                    self.read_char();
                    Token {
                        token_type: TokenType::Eq,
                        literal: "==".to_string(),
                        line: current_line,
                    }
                } else {
                    Token {
                        token_type: TokenType::Assign,
                        literal: "=".to_string(),
                        line: current_line,
                    }
                }
            }
            b'!' => {
                self.read_char();
                if self.ch == b'=' {
                    self.read_char();
                    Token {
                        token_type: TokenType::NotEq,
                        literal: "!=".to_string(),
                        line: current_line,
                    }
                } else {
                    let literal = "!".to_string();
                    Token {
                        token_type: TokenType::EOF,
                        literal,
                        line: current_line,
                    }
                }
            }
            b'>' => {
                self.read_char();
                if self.ch == b'=' {
                    self.read_char();
                    Token {
                        token_type: TokenType::GtEq,
                        literal: ">=".to_string(),
                        line: current_line,
                    }
                } else {
                    Token {
                        token_type: TokenType::Gt,
                        literal: ">".to_string(),
                        line: current_line,
                    }
                }
            }
            b'<' => {
                self.read_char();
                if self.ch == b'=' {
                    self.read_char();
                    Token {
                        token_type: TokenType::LtEq,
                        literal: "<=".to_string(),
                        line: current_line,
                    }
                } else {
                    Token {
                        token_type: TokenType::Lt,
                        literal: "<".to_string(),
                        line: current_line,
                    }
                }
            }
            b'+' => {
                let tok = Token {
                    token_type: TokenType::Plus,
                    literal: "+".to_string(),
                    line: current_line,
                };
                self.read_char();
                tok
            }
            b'-' => {
                let tok = Token {
                    token_type: TokenType::Minus,
                    literal: "-".to_string(),
                    line: current_line,
                };
                self.read_char();
                tok
            }
            b'*' => {
                let tok = Token {
                    token_type: TokenType::Star,
                    literal: "*".to_string(),
                    line: current_line,
                };
                self.read_char();
                tok
            }
            b'/' => {
                let tok = Token {
                    token_type: TokenType::Slash,
                    literal: "/".to_string(),
                    line: current_line,
                };
                self.read_char();
                tok
            }
            b'0'..=b'9' => {
                let literal = self.read_number();
                let value = literal.parse().unwrap_or(0);
                Token {
                    token_type: TokenType::Number(value),
                    literal,
                    line: current_line,
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let literal = self.read_ident();
                Token {
                    token_type: TokenType::Ident(literal.clone()),
                    literal,
                    line: current_line,
                }
            }
            0 => Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
                line: current_line,
            },
            _ => {
                let literal = format!("{}", self.ch as char);
                self.read_char();
                Token {
                    token_type: TokenType::EOF,
                    literal,
                    line: current_line,
                }
            }
        }
    }
}

pub fn tokenize(input: String) -> Result<Vec<Token>, ()> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        tokens.push(token.clone());

        if matches!(token.token_type, TokenType::EOF) {
            break;
        }
    }

    Ok(tokens)
}