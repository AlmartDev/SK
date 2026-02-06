use crate::evaluator::env::Environment;
use crate::evaluator::eval::Evaluator;
use crate::parser::lexer::TokenSpan;
use crate::core::error::Error;
use crate::core::value::Value;
use rand::Rng;

pub fn register(env: &mut Environment) {
    env.define("random".into(), Value::NativeFn(random));
    env.define("range".into(), Value::NativeFn(random_range));
    env.define("rangeInt".into(), Value::NativeFn(random_range_int));
}

fn err(token: TokenSpan, msg: String) -> Error {
    Error {
        token,
        message: msg
    }
}

pub fn random(_args: Vec<Value>, _span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    let mut rng = rand::rng();
    let random_value: f64 = rng.random(); 
    Ok(Value::Number(random_value)) 
}

// Supports both 2 numbers or one interval
pub fn random_range(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    if args.is_empty() || args.len() > 2 {
        return Err(err(span, "range() expects 1 or 2 arguments".to_string()));
    }

    let (min, max) = if args.len() == 1 {
        match &args[0] {
            Value::Interval(min, max) => (*min, *max),
            _ => return Err(err(span, "Single argument must be an interval".to_string())),
        }
    } else {
        let min = match &args[0] {
            Value::Number(n) => *n,
            _ => return Err(err(span, "First argument must be a number".to_string())),
        };
        let max = match &args[1] {
            Value::Number(n) => *n,
            _ => return Err(err(span, "Second argument must be a number".to_string())),
        };
        (min, max)
    };

    let mut rng = rand::rng();
    let random_value: f64 = rng.random_range(min..max);
    Ok(Value::Number(random_value))
}

pub fn random_range_int(args: Vec<Value>, span: TokenSpan, _: &mut Evaluator) -> Result<Value, Error> {
    if args.is_empty() || args.len() > 2 {
        return Err(err(span, "range_int() expects 1 or 2 arguments".to_string()));
    }

    let (min, max) = if args.len() == 1 {
        match &args[0] {
            Value::Interval(min, max) => (*min as i64, *max as i64),
            _ => return Err(err(span, "Single argument must be an interval".to_string())),
        }
    } else {
        let min = match &args[0] {
            Value::Number(n) => *n as i64,
            _ => return Err(err(span, "First argument must be a number".to_string())),
        };
        let max = match &args[1] {
            Value::Number(n) => *n as i64,
            _ => return Err(err(span, "Second argument must be a number".to_string())),
        };
        (min, max)
    };

    let mut rng = rand::rng();
    let random_value: i64 = rng.random_range(min..max);
    Ok(Value::Number(random_value as f64))
}