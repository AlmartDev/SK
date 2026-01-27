use core::fmt;

#[derive(Debug, Clone, PartialEq)]

pub enum SKBool {
    True,
    False,
    Partial
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(SKBool),
    Interval(f64, f64), // TODO: allow for any kind of value to be an interval not just numbers!
    Unknown,
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(SKBool::True) => write!(f, "true"),
            Value::Bool(SKBool::False) => write!(f, "false"),
            Value::Bool(SKBool::Partial) => write!(f, "partial"),
            Value::Interval(min, max) => write!(f, "[{}..{}]", min, max),
            Value::Unknown => write!(f, "unknown"),
            Value::None => write!(f, "none"),
        }
    }
}