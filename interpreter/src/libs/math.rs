use crate::core::value::Value;
use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::core::error::Error;
use crate::parser::lexer::{Token, TokenSpan};

// The SK Math library!

// Needed for every library
pub fn register(env: &mut Environment) {
    env.define("sqrt".into(), Value::NativeFn(sqrt));

    env.define("PI".into(), Value::Number(std::f64::consts::PI));
    env.define("E".into(), Value::Number(std::f64::consts::E));
}

fn err(msg: String) -> Error {
    Error {
        token: TokenSpan { token: Token::Unknown, line: 0, column: 0 },
        message: msg
    }
}

pub fn sqrt(args: Vec<Value>, _: &mut Evaluator) -> Result<Value, Error> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(n.sqrt())),
        Some(Value::Interval(min, max)) => {
            if *min < 0.0 {
                return Err(err("Cannot take sqrt of negative interval".to_string()));
            }
            Ok(Value::Interval(min.sqrt(), max.sqrt()))
        }
        _ => Err(err("sqrt() expects 1 number or interval".to_string())),
    }
}

// sin, cos, tan
// abs, min, max
// log, ln
// atan2, deg, rad