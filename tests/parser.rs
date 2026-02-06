#[cfg(test)]
mod tests {
    use pseudocu::ast::{Expr, Operator, Program};
    use pseudocu::lexer::tokenize;
    use pseudocu::parser::Parser;

    fn parse_expr(input: &str) -> Result<Expr, String> {
        let tokens = tokenize(input.to_string()).unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse_expression()
    }

    fn parse_program(input: &str) -> Result<Program, String> {
        let tokens = tokenize(input.to_string()).unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_single_number() {
        let result = parse_expr("42");
        assert_eq!(result, Ok(Expr::Number(42)));
    }

    #[test]
    fn test_simple_addition() {
        let result = parse_expr("1 + 2");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Number(1)),
                operator: Operator::Plus,
                right: Box::new(Expr::Number(2)),
            })
        );
    }

    #[test]
    fn test_simple_subtraction() {
        let result = parse_expr("5 - 3");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Number(5)),
                operator: Operator::Minus,
                right: Box::new(Expr::Number(3)),
            })
        );
    }

    #[test]
    fn test_simple_multiplication() {
        let result = parse_expr("2 * 3");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Number(2)),
                operator: Operator::Multiply,
                right: Box::new(Expr::Number(3)),
            })
        );
    }

    #[test]
    fn test_operator_precedence_mul_before_add() {
        // 2 + 3 * 4 should be parsed as 2 + (3 * 4)
        let result = parse_expr("2 + 3 * 4");
        
        let expected = Expr::BinaryOp {
            left: Box::new(Expr::Number(2)),
            operator: Operator::Plus,
            right: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Number(3)),
                operator: Operator::Multiply,
                right: Box::new(Expr::Number(4)),
            }),
        };
        
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_operator_precedence_add_before_sub() {
        // 5 - 2 + 3 should be parsed as (5 - 2) + 3 (left associative)
        let result = parse_expr("5 - 2 + 3");
        
        let inner = Expr::BinaryOp {
            left: Box::new(Expr::Number(5)),
            operator: Operator::Minus,
            right: Box::new(Expr::Number(2)),
        };
        
        let expected = Expr::BinaryOp {
            left: Box::new(inner),
            operator: Operator::Plus,
            right: Box::new(Expr::Number(3)),
        };
        
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_multiple_operations() {
        // 1 + 2 * 3 - 4
        let result = parse_expr("1 + 2 * 3 - 4");
        
        let mul = Expr::BinaryOp {
            left: Box::new(Expr::Number(2)),
            operator: Operator::Multiply,
            right: Box::new(Expr::Number(3)),
        };
        
        let add = Expr::BinaryOp {
            left: Box::new(Expr::Number(1)),
            operator: Operator::Plus,
            right: Box::new(mul),
        };
        
        let expected = Expr::BinaryOp {
            left: Box::new(add),
            operator: Operator::Minus,
            right: Box::new(Expr::Number(4)),
        };
        
        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_whitespace_ignored() {
        let result1 = parse_expr("1+2");
        let result2 = parse_expr("1 + 2");
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_parse_program_single_expression() {
        let result = parse_program("1 + 2");
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_error_on_invalid_input() {
        let result = parse_expr("+");
        assert!(result.is_err());
        println!("Error (expected): {}", result.err().unwrap());
    }

    #[test]
    fn test_error_on_empty_input() {
        let result = parse_expr("");
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_expression() {
        // Test: 10 * 2 + 15 - 3 * 4
        let result = parse_expr("10 * 2 + 15 - 3 * 4");
        
        // Should be parsed as: ((10 * 2) + 15) - (3 * 4)
        let left_mul = Expr::BinaryOp {
            left: Box::new(Expr::Number(10)),
            operator: Operator::Multiply,
            right: Box::new(Expr::Number(2)),
        };
        
        let add = Expr::BinaryOp {
            left: Box::new(left_mul),
            operator: Operator::Plus,
            right: Box::new(Expr::Number(15)),
        };
        
        let right_mul = Expr::BinaryOp {
            left: Box::new(Expr::Number(3)),
            operator: Operator::Multiply,
            right: Box::new(Expr::Number(4)),
        };
        
        let expected = Expr::BinaryOp {
            left: Box::new(add),
            operator: Operator::Minus,
            right: Box::new(right_mul),
        };
        
        assert_eq!(result, Ok(expected));
    }
}