use std::collections::HashMap;
use crate::ast::{Expr, Operator, Program, Statement};

pub struct Interpreter {
    variables: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self, program: Program) -> Result<(), String> {
        for statement in program.statements {
            self.execute_statement(statement)?;
        }
        Ok(())
    }

    fn execute_statement(&mut self, statement: Statement) -> Result<(), String> {
        match statement {
            Statement::Assign { name, value } => {
                let val = self.evaluate_expr(value)?;
                self.variables.insert(name, val);
            }
            Statement::Expr(expr) => {
                let _ = self.evaluate_expr(expr)?;
            }
        }
        Ok(())
    }

    fn evaluate_expr(&self, expr: Expr) -> Result<i64, String> {
        match expr {
            Expr::Number(val) => Ok(val),
            Expr::Identifier(name) => {
                self.variables.get(&name)
                    .copied()
                    .ok_or_else(|| format!("Undefined variable: {}", name))
            }
            Expr::BinaryOp { left, operator, right } => {
                let left_val = self.evaluate_expr(*left)?;
                let right_val = self.evaluate_expr(*right)?;
                
                match operator {
                    Operator::Plus => Ok(left_val + right_val),
                    Operator::Minus => Ok(left_val - right_val),
                    Operator::Multiply => Ok(left_val * right_val),
                    Operator::Divide => {
                        if right_val == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    Operator::Eq => Ok(if left_val == right_val { 1 } else { 0 }),
                    Operator::NotEq => Ok(if left_val != right_val { 1 } else { 0 }),
                    Operator::Gt => Ok(if left_val > right_val { 1 } else { 0 }),
                    Operator::Lt => Ok(if left_val < right_val { 1 } else { 0 }),
                    Operator::GtEq => Ok(if left_val >= right_val { 1 } else { 0 }),
                    Operator::LtEq => Ok(if left_val <= right_val { 1 } else { 0 }),
                }
            }
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<i64> {
        self.variables.get(name).copied()
    }

    pub fn print_variables(&self) {
        let mut vars: Vec<_> = self.variables.iter().collect();
        vars.sort_by(|a, b| a.0.cmp(b.0));
        for (name, value) in vars {
            println!("{} = {}", name, value);
        }
    }
}
