#[cfg(test)]
mod tests {
    use pseudocu::ast::{Expr, Operator, Program, Statement};
    use pseudocu::interpreter::Interpreter;

    fn run_program(statements: Vec<Statement>) -> Interpreter {
        let program = Program { statements };
        let mut interpreter = Interpreter::new();
        interpreter.run(program).unwrap();
        interpreter
    }

    // ==================== Assignment Tests ====================

    #[test]
    fn test_assign_number() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "x".to_string(),
                value: Expr::Number(42),
            }
        ]);
        assert_eq!(interpreter.get_variable("x"), Some(42));
    }

    #[test]
    fn test_assign_multiple_variables() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "a".to_string(),
                value: Expr::Number(10),
            },
            Statement::Assign {
                name: "b".to_string(),
                value: Expr::Number(20),
            },
            Statement::Assign {
                name: "c".to_string(),
                value: Expr::Number(30),
            },
        ]);
        assert_eq!(interpreter.get_variable("a"), Some(10));
        assert_eq!(interpreter.get_variable("b"), Some(20));
        assert_eq!(interpreter.get_variable("c"), Some(30));
    }

    // ==================== Arithmetic Expression Tests ====================

    #[test]
    fn test_addition() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(5)),
                    operator: Operator::Plus,
                    right: Box::new(Expr::Number(3)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(8));
    }

    #[test]
    fn test_subtraction() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(10)),
                    operator: Operator::Minus,
                    right: Box::new(Expr::Number(4)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(6));
    }

    #[test]
    fn test_multiplication() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(6)),
                    operator: Operator::Multiply,
                    right: Box::new(Expr::Number(7)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(42));
    }

    #[test]
    fn test_division() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(20)),
                    operator: Operator::Divide,
                    right: Box::new(Expr::Number(4)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(5));
    }

    #[test]
    fn test_division_by_zero_error() {
        let program = Program {
            statements: vec![
                Statement::Assign {
                    name: "result".to_string(),
                    value: Expr::BinaryOp {
                        left: Box::new(Expr::Number(10)),
                        operator: Operator::Divide,
                        right: Box::new(Expr::Number(0)),
                    },
                }
            ],
        };
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(program);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }

    #[test]
    fn test_operator_precedence() {
        // 2 + 3 * 4 = 2 + 12 = 14
        let mul = Expr::BinaryOp {
            left: Box::new(Expr::Number(3)),
            operator: Operator::Multiply,
            right: Box::new(Expr::Number(4)),
        };
        let add = Expr::BinaryOp {
            left: Box::new(Expr::Number(2)),
            operator: Operator::Plus,
            right: Box::new(mul),
        };

        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: add,
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(14));
    }

    // ==================== Variable Reference Tests ====================

    #[test]
    fn test_use_variable_in_expression() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "x".to_string(),
                value: Expr::Number(10),
            },
            Statement::Assign {
                name: "y".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("x".to_string())),
                    operator: Operator::Plus,
                    right: Box::new(Expr::Number(5)),
                },
            },
        ]);
        assert_eq!(interpreter.get_variable("x"), Some(10));
        assert_eq!(interpreter.get_variable("y"), Some(15));
    }

    #[test]
    fn test_two_variables_in_expression() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "a".to_string(),
                value: Expr::Number(7),
            },
            Statement::Assign {
                name: "b".to_string(),
                value: Expr::Number(3),
            },
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("a".to_string())),
                    operator: Operator::Plus,
                    right: Box::new(Expr::Identifier("b".to_string())),
                },
            },
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(10));
    }

    #[test]
    fn test_example_from_user() {
        // d = 5, a = 4, result = d + a
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "d".to_string(),
                value: Expr::Number(5),
            },
            Statement::Assign {
                name: "a".to_string(),
                value: Expr::Number(4),
            },
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("d".to_string())),
                    operator: Operator::Plus,
                    right: Box::new(Expr::Identifier("a".to_string())),
                },
            },
        ]);
        assert_eq!(interpreter.get_variable("d"), Some(5));
        assert_eq!(interpreter.get_variable("a"), Some(4));
        assert_eq!(interpreter.get_variable("result"), Some(9));
    }

    #[test]
    fn test_undefined_variable_error() {
        let program = Program {
            statements: vec![
                Statement::Assign {
                    name: "result".to_string(),
                    value: Expr::BinaryOp {
                        left: Box::new(Expr::Identifier("undefined".to_string())),
                        operator: Operator::Plus,
                        right: Box::new(Expr::Number(5)),
                    },
                }
            ],
        };
        let mut interpreter = Interpreter::new();
        let result = interpreter.run(program);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Undefined variable"));
    }

    #[test]
    fn test_variable_reassignment() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "x".to_string(),
                value: Expr::Number(10),
            },
            Statement::Assign {
                name: "x".to_string(),
                value: Expr::Number(20),
            },
        ]);
        assert_eq!(interpreter.get_variable("x"), Some(20));
    }

    #[test]
    fn test_chain_assignments() {
        // a = 5, b = a + 3, c = b * 2
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "a".to_string(),
                value: Expr::Number(5),
            },
            Statement::Assign {
                name: "b".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("a".to_string())),
                    operator: Operator::Plus,
                    right: Box::new(Expr::Number(3)),
                },
            },
            Statement::Assign {
                name: "c".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("b".to_string())),
                    operator: Operator::Multiply,
                    right: Box::new(Expr::Number(2)),
                },
            },
        ]);
        assert_eq!(interpreter.get_variable("a"), Some(5));
        assert_eq!(interpreter.get_variable("b"), Some(8));
        assert_eq!(interpreter.get_variable("c"), Some(16));
    }

    // ==================== Comparison Operator Tests ====================

    #[test]
    fn test_comparison_equal_true() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(5)),
                    operator: Operator::Eq,
                    right: Box::new(Expr::Number(5)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(1)); // true = 1
    }

    #[test]
    fn test_comparison_equal_false() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(5)),
                    operator: Operator::Eq,
                    right: Box::new(Expr::Number(10)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(0)); // false = 0
    }

    #[test]
    fn test_comparison_not_equal_true() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(5)),
                    operator: Operator::NotEq,
                    right: Box::new(Expr::Number(10)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(1));
    }

    #[test]
    fn test_comparison_not_equal_false() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(7)),
                    operator: Operator::NotEq,
                    right: Box::new(Expr::Number(7)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(0));
    }

    #[test]
    fn test_comparison_greater_than_true() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(10)),
                    operator: Operator::Gt,
                    right: Box::new(Expr::Number(5)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(1));
    }

    #[test]
    fn test_comparison_greater_than_false() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(3)),
                    operator: Operator::Gt,
                    right: Box::new(Expr::Number(5)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(0));
    }

    #[test]
    fn test_comparison_less_than_true() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(2)),
                    operator: Operator::Lt,
                    right: Box::new(Expr::Number(8)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(1));
    }

    #[test]
    fn test_comparison_less_than_false() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(10)),
                    operator: Operator::Lt,
                    right: Box::new(Expr::Number(5)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(0));
    }

    #[test]
    fn test_comparison_greater_than_or_equal_true() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(5)),
                    operator: Operator::GtEq,
                    right: Box::new(Expr::Number(5)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(1));
    }

    #[test]
    fn test_comparison_less_than_or_equal_true() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Number(5)),
                    operator: Operator::LtEq,
                    right: Box::new(Expr::Number(5)),
                },
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(1));
    }

    #[test]
    fn test_comparison_with_variables() {
        let interpreter = run_program(vec![
            Statement::Assign {
                name: "a".to_string(),
                value: Expr::Number(10),
            },
            Statement::Assign {
                name: "b".to_string(),
                value: Expr::Number(20),
            },
            Statement::Assign {
                name: "result".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("a".to_string())),
                    operator: Operator::Lt,
                    right: Box::new(Expr::Identifier("b".to_string())),
                },
            },
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(1));
    }

    // ==================== Complex Expression Tests ====================

    #[test]
    fn test_complex_arithmetic_expression() {
        // result = 10 * 2 + 15 - 3 * 4 = 20 + 15 - 12 = 23
        let mul1 = Expr::BinaryOp {
            left: Box::new(Expr::Number(10)),
            operator: Operator::Multiply,
            right: Box::new(Expr::Number(2)),
        };
        let mul2 = Expr::BinaryOp {
            left: Box::new(Expr::Number(3)),
            operator: Operator::Multiply,
            right: Box::new(Expr::Number(4)),
        };
        let add = Expr::BinaryOp {
            left: Box::new(mul1),
            operator: Operator::Plus,
            right: Box::new(Expr::Number(15)),
        };
        let sub = Expr::BinaryOp {
            left: Box::new(add),
            operator: Operator::Minus,
            right: Box::new(mul2),
        };

        let interpreter = run_program(vec![
            Statement::Assign {
                name: "result".to_string(),
                value: sub,
            }
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(23));
    }

    #[test]
    fn test_nested_expression_with_variables() {
        // a = 2, b = 3, c = 4
        // result = a * b + c * (a + b) = 6 + 4 * 5 = 6 + 20 = 26
        let add = Expr::BinaryOp {
            left: Box::new(Expr::Identifier("a".to_string())),
            operator: Operator::Plus,
            right: Box::new(Expr::Identifier("b".to_string())),
        };
        let mul1 = Expr::BinaryOp {
            left: Box::new(Expr::Identifier("a".to_string())),
            operator: Operator::Multiply,
            right: Box::new(Expr::Identifier("b".to_string())),
        };
        let mul2 = Expr::BinaryOp {
            left: Box::new(Expr::Identifier("c".to_string())),
            operator: Operator::Multiply,
            right: Box::new(add),
        };
        let result = Expr::BinaryOp {
            left: Box::new(mul1),
            operator: Operator::Plus,
            right: Box::new(mul2),
        };

        let interpreter = run_program(vec![
            Statement::Assign {
                name: "a".to_string(),
                value: Expr::Number(2),
            },
            Statement::Assign {
                name: "b".to_string(),
                value: Expr::Number(3),
            },
            Statement::Assign {
                name: "c".to_string(),
                value: Expr::Number(4),
            },
            Statement::Assign {
                name: "result".to_string(),
                value: result,
            },
        ]);
        assert_eq!(interpreter.get_variable("result"), Some(26));
    }
}
