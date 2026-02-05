use pseudocu::lexer::{Lexer, Token, TokenType};

#[test]
fn test_basic_expression() {
    let input = "1 + 2 * 3".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Number(1),
        literal: "1".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Plus,
        literal: "+".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Number(2),
        literal: "2".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Star,
        literal: "*".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Number(3),
        literal: "3".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
        line: 1,
    });
}

#[test]
fn test_whitespace_and_newlines() {
    let input = "10\n+\t20 ".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Number(10),
        literal: "10".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Plus,
        literal: "+".to_string(),
        line: 2,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Number(20),
        literal: "20".to_string(),
        line: 2,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
        line: 2,
    });
}