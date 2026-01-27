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
                    Expr::Variable { name: Token::Kind } => "kind".to_string(), // Add this
                    _ => return Err("Only identifiers can be called as functions".to_string()),
                };
                
                let mut eval_args = Vec::new();
                for arg in arguments {
                    eval_args.push(self.eval_expr(arg)?);
                }

                match name.as_str() {   // more primitive functions to be added here!
                    "print" => {
                        for arg in eval_args {
                            println!("{}", arg);
                        }
                        Ok(Value::None)
                    },
                    "kind" => {
                        if eval_args.len() != 1 {
                            return Err("kind() expects exactly 1 argument".to_string());
                        }
                        let type_name = match eval_args[0] { // symbolic, quiet ...
                            Value::Number(_) => "Number",
                            Value::String(_) => "String",
                            Value::Bool(_) => "Bool",
                            Value::Interval(_, _) => "Interval",
                            Value::Unknown => "Unknown",
                            Value::None => "None",
                        };
                        Ok(Value::String(type_name.to_string()))
                    },
                    _ => Err(format!("Undefined function '{}'", name))
                }
            }
        }
    }

    fn apply_binary(&self, left: Value, op: Token, right: Value) -> Result<Value, String> {
        match (left.clone(), op.clone(), right.clone()) {
            // --- 1. Identity & Algebraic Simplification ---
            // Multiplication by Zero: 0 * unknown = 0
            (Value::Number(n), Token::Star, _) if n == 0.0 => Ok(Value::Number(0.0)),
            (_, Token::Star, Value::Number(n)) if n == 0.0 => Ok(Value::Number(0.0)),

            // Self-Subtraction: x - x = 0 (even if x is unknown or an interval)
            (l, Token::Minus, r) if l == r && l != Value::Unknown => Ok(Value::Number(0.0)),
            
            // Division by Self: x / x = 1 (excluding zero/unknown/intervals containing zero)
            (l, Token::Slash, r) if l == r => {
                match l {
                    Value::Number(n) if n != 0.0 => Ok(Value::Number(1.0)),
                    Value::Interval(min, max) if min > 0.0 || max < 0.0 => Ok(Value::Number(1.0)),
                    _ => Ok(Value::Unknown), // Could be 0/0
                }
            }

            // --- 2. Existing Unknown Propagation ---
            (Value::Unknown, _, _) | (_, _, Value::Unknown) => Ok(Value::Unknown),

            // --- 3. Number & Number Arithmetic ---
            (Value::Number(a), Token::Plus, Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Number(a), Token::Minus, Value::Number(b)) => Ok(Value::Number(a - b)),
            (Value::Number(a), Token::Star, Value::Number(b)) => Ok(Value::Number(a * b)),
            (Value::Number(a), Token::Slash, Value::Number(b)) => {
                if b == 0.0 { return Err("Division by zero".to_string()); }
                Ok(Value::Number(a / b))
            },

            // --- 4. Interval Arithmetic ---
            // Interval & Number
            (Value::Interval(min, max), Token::Plus, Value::Number(n)) |
            (Value::Number(n), Token::Plus, Value::Interval(min, max)) => Ok(Value::Interval(min + n, max + n)),
            
            (Value::Interval(min, max), Token::Minus, Value::Number(n)) => Ok(Value::Interval(min - n, max - n)),
            (Value::Number(n), Token::Minus, Value::Interval(min, max)) => Ok(Value::Interval(n - max, n - min)),

            (Value::Interval(min, max), Token::Star, Value::Number(n)) |
            (Value::Number(n), Token::Star, Value::Interval(min, max)) => {
                let a = min * n;
                let b = max * n;
                Ok(Value::Interval(a.min(b), a.max(b)))
            },

            // Interval & Interval
            (Value::Interval(min1, max1), Token::Plus, Value::Interval(min2, max2)) => {
                Ok(Value::Interval(min1 + min2, max1 + max2))
            },
            (Value::Interval(min1, max1), Token::Minus, Value::Interval(min2, max2)) => {
                Ok(Value::Interval(min1 - max2, max1 - min2))
            },
            (Value::Interval(min1, max1), Token::Star, Value::Interval(min2, max2)) => {
                let p = [min1 * min2, min1 * max2, max1 * min2, max1 * max2];
                Ok(Value::Interval(
                    p.iter().copied().fold(f64::INFINITY, f64::min),
                    p.iter().copied().fold(f64::NEG_INFINITY, f64::max)
                ))
            },

            _ => Err("Operation not supported for these types".to_string()),
        }
    }
}