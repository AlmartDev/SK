use std::path::Path;
use std::fs;

use crate::parser::lexer::Token;

pub mod parser;

pub struct SKInterpreter;

impl SKInterpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, source: &Path) -> Result<String, String> {
        let raw = fs::read_to_string(source).expect("Couldn't open file!");

        let tokens = parser::lexer::tokenize(raw);
        
        // debug
        for token in tokens {
            if token.token == Token::NewLine {
                print!("\n")
            } else {
                print!("{:?} ", token.token);
            }
        }
        println!("");

        //let program = parser::ast::parse(tokens); // + ast

        // evaluator::eval::evaluate(ast, &mut self.env)

        // placeholder
        Ok("output".to_string())
    }
}
