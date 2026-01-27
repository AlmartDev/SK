use std::path::Path;
use std::fs;

pub mod core;
pub mod parser;
pub mod evaluator;

use crate::parser::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::evaluator::eval::Evaluator;
use crate::evaluator::env::Environment;
use crate::core::value::Value;

// only needed for debug
use crate::parser::ast::Stmt;

pub struct SKInterpreter {
    env: Environment,
}

impl SKInterpreter {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    pub fn execute(&mut self, source: &Path) -> Result<Value, String> {
        let raw = fs::read_to_string(source).map_err(|e| e.to_string())?;

        let mut lexer = Lexer::new(raw);
        let tokens = lexer.tokenize()?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse()?;

        //self._debug_ast(&ast);

        let mut evaluator = Evaluator::new(&mut self.env);
        evaluator.evaluate(ast)
    }

    fn _debug_ast(&self, program: &Vec<Stmt>) {
        println!("--- Abstract Syntax Tree ---");
        for (i, stmt) in program.iter().enumerate() {
            println!("Statement {}:", i);
            println!("{:#?}", stmt);
            println!("-----------------------");
        }
    }
}