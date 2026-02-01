use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    pub token: crate::parser::lexer::TokenSpan,
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(line {}): {}", self.token.line, self.message)
    }
}