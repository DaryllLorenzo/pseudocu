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

// ==================== Identifier Tests ====================

#[test]
fn test_identifier() {
    let input = "x".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Ident("x".to_string()),
        literal: "x".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
        line: 1,
    });
}

#[test]
fn test_identifier_with_underscore() {
    let input = "my_var".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Ident("my_var".to_string()),
        literal: "my_var".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
        line: 1,
    });
}

#[test]
fn test_identifier_with_numbers() {
    let input = "var123".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Ident("var123".to_string()),
        literal: "var123".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
        line: 1,
    });
}

#[test]
fn test_identifier_uppercase() {
    let input = "MyVariable".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Ident("MyVariable".to_string()),
        literal: "MyVariable".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
        line: 1,
    });
}

// ==================== Assignment Operator Tests ====================

#[test]
fn test_assignment_operator() {
    let input = "x = 5".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Ident("x".to_string()),
        literal: "x".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Assign,
        literal: "=".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Number(5),
        literal: "5".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
        line: 1,
    });
}

// ==================== Division Operator Tests ====================

#[test]
fn test_division_operator() {
    let input = "10 / 2".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Number(10),
        literal: "10".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Slash,
        literal: "/".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Number(2),
        literal: "2".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
        line: 1,
    });
}

// ==================== Comparison Operator Tests ====================

#[test]
fn test_equal_comparison() {
    let input = "x == y".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Ident("x".to_string()),
        literal: "x".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Eq,
        literal: "==".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Ident("y".to_string()),
        literal: "y".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
        line: 1,
    });
}

#[test]
fn test_not_equal_comparison() {
    let input = "a != b".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Ident("a".to_string()),
        literal: "a".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::NotEq,
        literal: "!=".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Ident("b".to_string()),
        literal: "b".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
        line: 1,
    });
}

#[test]
fn test_greater_than() {
    let input = "5 > 3".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Number(5),
        literal: "5".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Gt,
        literal: ">".to_string(),
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
fn test_less_than() {
    let input = "2 < 10".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Number(2),
        literal: "2".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Lt,
        literal: "<".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Number(10),
        literal: "10".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
        line: 1,
    });
}

#[test]
fn test_greater_than_or_equal() {
    let input = "x >= 5".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Ident("x".to_string()),
        literal: "x".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::GtEq,
        literal: ">=".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Number(5),
        literal: "5".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
        line: 1,
    });
}

#[test]
fn test_less_than_or_equal() {
    let input = "y <= 100".to_string();
    let mut lexer = Lexer::new(input);

    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Ident("y".to_string()),
        literal: "y".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::LtEq,
        literal: "<=".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::Number(100),
        literal: "100".to_string(),
        line: 1,
    });
    assert_eq!(lexer.next_token(), Token {
        token_type: TokenType::EOF,
        literal: "".to_string(),
        line: 1,
    });
}

// ==================== Complete Program Test ====================

#[test]
fn test_complete_assignment_program() {
    let input = "d = 5\na = 4\nresult = d + a".to_string();
    let mut lexer = Lexer::new(input);

    // d = 5
    assert_eq!(lexer.next_token().token_type, TokenType::Ident("d".to_string()));
    assert_eq!(lexer.next_token().token_type, TokenType::Assign);
    assert_eq!(lexer.next_token().token_type, TokenType::Number(5));
    
    // a = 4
    assert_eq!(lexer.next_token().token_type, TokenType::Ident("a".to_string()));
    assert_eq!(lexer.next_token().token_type, TokenType::Assign);
    assert_eq!(lexer.next_token().token_type, TokenType::Number(4));
    
    // result = d + a
    assert_eq!(lexer.next_token().token_type, TokenType::Ident("result".to_string()));
    assert_eq!(lexer.next_token().token_type, TokenType::Assign);
    assert_eq!(lexer.next_token().token_type, TokenType::Ident("d".to_string()));
    assert_eq!(lexer.next_token().token_type, TokenType::Plus);
    assert_eq!(lexer.next_token().token_type, TokenType::Ident("a".to_string()));
    
    assert_eq!(lexer.next_token().token_type, TokenType::EOF);
}