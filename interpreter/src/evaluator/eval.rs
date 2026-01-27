use crate::parser::ast::{Expr, Stmt};
use crate::parser::lexer::Token;
use crate::core::value::{Value, SKBool};
use crate::evaluator::env::Environment;

pub struct Evaluator<'a> {
    env: &'a mut Environment,
}

impl<'a> Evaluator<'a> {
    pub fn new(env: &'a mut Environment) -> Self {
        Self { env }
    }

    pub fn evaluate(&mut self, statements: Vec<Stmt>) -> Result<Value, String> {
        let mut last_value = Value::None;
        for stmt in statements {
            last_value = self.eval_stmt(stmt)?;
        }
        Ok(last_value)
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Let { name, initializer } => {
                let val = self.eval_expr(initializer)?;
                if let Token::Identifier(n) = name {
                    self.env.define(n, val);
                }
                Ok(Value::None)
            }
            Stmt::Print { expression } => {
                let val = self.eval_expr(expression)?;
                println!("{}", val);
                Ok(Value::None)
            }
            Stmt::Panic => {
                eprintln!("Program panicked!");
                std::process::exit(1);
            }
            Stmt::Expression { expression } => self.eval_expr(expression),
            Stmt::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                if let Token::Identifier(n) = name {
                    self.env.define(n, val);
                }
                Ok(Value::None)
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal { value } => match value {
                Token::Number(n) => Ok(Value::Number(n)),
                Token::String(s) => Ok(Value::String(s)),
                Token::True => Ok(Value::Bool(SKBool::True)),
                Token::False => Ok(Value::Bool(SKBool::False)),
                Token::Partial => Ok(Value::Bool(SKBool::Partial)),
                Token::Unknown => Ok(Value::Unknown),
                Token::None => Ok(Value::None),
                _ => Err(format!("Unsupported literal: {:?}", value)),
            },

            Expr::Variable { name } => {
                if let Token::Identifier(n) = name {
                    self.env.get(&n)
                } else {
                    Err("Invalid identifier for variable lookup".to_string())
                }
            },

            Expr::Interval { min, max } => {
                let low = self.eval_expr(*min)?;
                let high = self.eval_expr(*max)?;
                match (low, high) {
                    (Value::Number(l), Value::Number(h)) => Ok(Value::Interval(l, h)),
                    _ => Err("Interval bounds must be numbers".to_string()),
                }
            },

            Expr::Binary { left, operator, right } => {
                let l_val = self.eval_expr(*left)?;
                let r_val = self.eval_expr(*right)?;
                self.apply_binary(l_val, operator, r_val)
            },

            Expr::Unary { operator, right } => {
                let val = self.eval_expr(*right)?;
                match (operator, val) {
                    (Token::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                    (Token::Not, Value::Bool(SKBool::True)) => Ok(Value::Bool(SKBool::False)),
                    (Token::Not, Value::Bool(SKBool::False)) => Ok(Value::Bool(SKBool::True)),
                    _ => Err("Invalid unary operation".to_string()),
                }
            },

            Expr::Grouping { expression } => self.eval_expr(*expression),
                Expr::Call { callee, arguments } => {
                let name = match *callee {
                    Expr::Variable { name: Token::Identifier(n) } => n,
                    Expr::Variable { name: Token::Print } => "print".to_string(),
                    _ => return Err("Only identifiers can be called as functions".to_string()),
                };
                
                let mut eval_args = Vec::new();
                for arg in arguments {
                    eval_args.push(self.eval_expr(arg)?);
                }

                if name == "print" {
                    for arg in eval_args {
                        println!("{}", arg);
                    }
                    Ok(Value::None)
                } else {
                    Err(format!("Undefined function '{}'", name))
                }
            },
        }
    }

    fn apply_binary(&self, left: Value, op: Token, right: Value) -> Result<Value, String> {
        match (left, op, right) {
            (Value::Unknown, _, _) | (_, _, Value::Unknown) => Ok(Value::Unknown),
            (Value::Number(a), Token::Plus, Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Number(a), Token::Minus, Value::Number(b)) => Ok(Value::Number(a - b)),
            (Value::Number(a), Token::Star, Value::Number(b)) => Ok(Value::Number(a * b)),
            (Value::Number(a), Token::Slash, Value::Number(b)) => {
                if b == 0.0 { return Err("Division by zero".to_string()); }
                Ok(Value::Number(a / b))
            },
            (Value::Number(a), Token::Caret, Value::Number(b)) => Ok(Value::Number(a.powf(b))),
            (Value::Number(a), Token::Greater, Value::Number(b)) => Ok(Value::Bool(if a > b { SKBool::True } else { SKBool::False })),
            (Value::Number(a), Token::Less, Value::Number(b)) => Ok(Value::Bool(if a < b { SKBool::True } else { SKBool::False })),
            (Value::Number(a), Token::Equal, Value::Number(b)) => Ok(Value::Bool(if a == b { SKBool::True } else { SKBool::False })),
            _ => Err("Operation not supported for these types".to_string()),
        }
    }
}