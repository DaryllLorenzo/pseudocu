#[cfg(test)]
mod tests {
    use pseudocu::ast::{Expr, Operator, Program, Statement};
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
    fn test_simple_division() {
        let result = parse_expr("10 / 2");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Number(10)),
                operator: Operator::Divide,
                right: Box::new(Expr::Number(2)),
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

    // ==================== Identifier Tests ====================

    #[test]
    fn test_identifier_in_expression() {
        let result = parse_expr("x + 5");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("x".to_string())),
                operator: Operator::Plus,
                right: Box::new(Expr::Number(5)),
            })
        );
    }

    #[test]
    fn test_two_identifiers() {
        let result = parse_expr("a + b");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("a".to_string())),
                operator: Operator::Plus,
                right: Box::new(Expr::Identifier("b".to_string())),
            })
        );
    }

    #[test]
    fn test_identifier_with_underscore() {
        let result = parse_expr("my_var + 10");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("my_var".to_string())),
                operator: Operator::Plus,
                right: Box::new(Expr::Number(10)),
            })
        );
    }

    #[test]
    fn test_identifier_with_numbers() {
        let result = parse_expr("var1 + var2");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("var1".to_string())),
                operator: Operator::Plus,
                right: Box::new(Expr::Identifier("var2".to_string())),
            })
        );
    }

    // ==================== Assignment Tests ====================

    #[test]
    fn test_assignment_number() {
        let result = parse_program("x = 5");
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::Assign {
                name: "x".to_string(),
                value: Expr::Number(5),
            }
        );
    }

    #[test]
    fn test_assignment_expression() {
        let result = parse_program("result = a + b");
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.statements.len(), 1);
        assert_eq!(
            program.statements[0],
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("a".to_string())),
                    operator: Operator::Plus,
                    right: Box::new(Expr::Identifier("b".to_string())),
                },
            }
        );
    }

    #[test]
    fn test_multiple_assignments() {
        let result = parse_program("d = 5\na = 4\nresult = d + a");
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.statements.len(), 3);

        assert_eq!(
            program.statements[0],
            Statement::Assign {
                name: "d".to_string(),
                value: Expr::Number(5),
            }
        );
        assert_eq!(
            program.statements[1],
            Statement::Assign {
                name: "a".to_string(),
                value: Expr::Number(4),
            }
        );
        assert_eq!(
            program.statements[2],
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("d".to_string())),
                    operator: Operator::Plus,
                    right: Box::new(Expr::Identifier("a".to_string())),
                },
            }
        );
    }

    // ==================== Comparison Operators Tests ====================

    #[test]
    fn test_equal_comparison() {
        let result = parse_expr("x == y");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("x".to_string())),
                operator: Operator::Eq,
                right: Box::new(Expr::Identifier("y".to_string())),
            })
        );
    }

    #[test]
    fn test_not_equal_comparison() {
        let result = parse_expr("a != b");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("a".to_string())),
                operator: Operator::NotEq,
                right: Box::new(Expr::Identifier("b".to_string())),
            })
        );
    }

    #[test]
    fn test_greater_than() {
        let result = parse_expr("5 > 3");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Number(5)),
                operator: Operator::Gt,
                right: Box::new(Expr::Number(3)),
            })
        );
    }

    #[test]
    fn test_less_than() {
        let result = parse_expr("2 < 10");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Number(2)),
                operator: Operator::Lt,
                right: Box::new(Expr::Number(10)),
            })
        );
    }

    #[test]
    fn test_greater_than_or_equal() {
        let result = parse_expr("x >= 5");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("x".to_string())),
                operator: Operator::GtEq,
                right: Box::new(Expr::Number(5)),
            })
        );
    }

    #[test]
    fn test_less_than_or_equal() {
        let result = parse_expr("y <= 100");
        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("y".to_string())),
                operator: Operator::LtEq,
                right: Box::new(Expr::Number(100)),
            })
        );
    }

    #[test]
    fn test_operator_precedence_comparison() {
        // a + b == c * d should be (a + b) == (c * d)
        let result = parse_expr("a + b == c * d");
        
        let left = Expr::BinaryOp {
            left: Box::new(Expr::Identifier("a".to_string())),
            operator: Operator::Plus,
            right: Box::new(Expr::Identifier("b".to_string())),
        };
        
        let right = Expr::BinaryOp {
            left: Box::new(Expr::Identifier("c".to_string())),
            operator: Operator::Multiply,
            right: Box::new(Expr::Identifier("d".to_string())),
        };

        assert_eq!(
            result,
            Ok(Expr::BinaryOp {
                left: Box::new(left),
                operator: Operator::Eq,
                right: Box::new(right),
            })
        );
    }
}