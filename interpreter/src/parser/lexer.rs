#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    UnknownChar(char),
    Identifier(String),

    Number(f64),
    String(String),

    // Keywords
    Let,
    Const,
    Unknown,
    Symbolic,
    Quiet,
    If,
    Elif,
    Else,
    Merge,
    Strict,
    None,
    Print,
    Input,
    Panic,
    Fn,
    Return,
    For,
    While,
    
    // Operators & Symbols
    Assign,      // =
    Arrow,       // ->
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Caret,       // ^
    RangeSep,    // ..
    
    // Delimiters
    LParen,      // (
    RParen,      // )
    LBracket,    // [
    RBracket,    // ]
    LBrace,      // {
    RBrace,      // }
    Quote,       // Either " or '

    Greater,     // >
    Less,        // <
    GreaterEqual,// >=
    LessEqual,   // <=
    Equal,       // ==
    NotEqual,    // !=
    Not,         // !

    NewLine,
    EOF,
}

#[derive(Debug, Clone)]
pub struct TokenSpan {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

pub fn tokenize(raw: String) -> Vec<TokenSpan> {
    let mut lexer = Lexer::new(raw);
    lexer.run()
}

struct Lexer {
    source: Vec<char>,
    cursor: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Self {
            source: input.chars().collect(),
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    fn run(&mut self) -> Vec<TokenSpan> {
        let mut tokens = Vec::new();
        while !self.is_at_end() {
            if let Some(token) = self.next_token() {
                tokens.push(TokenSpan {
                    token,
                    line: self.line,
                    column: self.column
                });
            }
        }
        tokens.push(TokenSpan { token: Token::EOF , line: self.line, column: self.column });
        tokens
    }

    fn is_at_end(&self) -> bool {
        self.cursor >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() { '\0' } else { self.source[self.cursor] }
    }

    fn peek_next(&self) -> char {
        if self.cursor + 1 >= self.source.len() { '\0' } else { self.source[self.cursor + 1] }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.cursor];
        self.cursor += 1;
        self.column += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.cursor] != expected {
            return false;
        }
        self.cursor += 1;
        self.column += 1;
        true
    }

    fn next_token(&mut self) -> Option<Token> {
        let c = self.advance();

        match c {
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '[' => Some(Token::LBracket),
            ']' => Some(Token::RBracket),
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            '+' => Some(Token::Plus),
            '*' => Some(Token::Star),
            '^' => Some(Token::Caret),
            
            '/' => {
                if self.match_char('/') {   // if comment, continue!
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    None
                } else {
                    Some(Token::Slash)
                }
            }

            '\n' => {
                self.line += 1;
                self.column = 1;

                // we shouldn't take 2 newlines together as that => empty operation, so we skip it
                while self.peek() == '\n' || self.peek() == '\r' {
                    let next_c = self.advance();
                    if next_c == '\n' {
                        self.line += 1;
                        self.column = 1;
                    }
                }

                Some(Token::NewLine)
            }

            '=' => {
                if self.match_char('=') {
                    Some(Token::Equal)
                } else {
                    Some(Token::Assign)
                }
            }
            '-' => {
                if self.match_char('>') {
                    Some(Token::Arrow)
                } else {
                    Some(Token::Minus)
                }
            }
            '.' => {
                if self.match_char('.') {
                    Some(Token::RangeSep)
                } else {
                    Some(Token::UnknownChar('.'))
                }
            }

            '>' => {
                if self.match_char('=') { Some(Token::GreaterEqual) } 
                else { Some(Token::Greater) }
            }
            '<' => {
                if self.match_char('=') { Some(Token::LessEqual) } 
                else { Some(Token::Less) }
            }
            '!' => {
                if self.match_char('=') { Some(Token::NotEqual) } 
                else { Some(Token::Not) }
            }

            // Whitespace
            ' ' | '\r' | '\t' => None,            

            '"' | '\'' => Some(self.string(c)),
            
            _ => {
                if c.is_ascii_digit() {
                    Some(self.number())
                } else if c.is_alphabetic() || c == '_' {
                    Some(self.identifier())
                } else {
                    Some(Token::UnknownChar(c))
                }
            }
        }
    }

    fn identifier(&mut self) -> Token {
        let mut text = String::new();
        text.push(self.source[self.cursor - 1]);

        while self.peek().is_alphanumeric() || self.peek() == '_' {
            text.push(self.advance());
        }

        match text.as_str() {
            "let" => Token::Let,
            "const" => Token::Const,
            "unknown" => Token::Unknown,
            "symbolic" => Token::Symbolic,
            "quiet" => Token::Quiet,
            "if" => Token::If,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "merge" => Token::Merge,
            "strict" => Token::Strict,
            "print" => Token::Print,
            "input" => Token::Input,
            "panic" => Token::Panic,
            "fn" => Token::Fn,
            "return" => Token::Return,
            "for" => Token::For,
            "while" => Token::While,
            "none" => Token::None,
            _ => Token::Identifier(text),
        }
    }

    fn number(&mut self) -> Token {
        let mut text = String::new();
        text.push(self.source[self.cursor - 1]);

        while self.peek().is_ascii_digit() {
            text.push(self.advance());
        }

        // decimal logic
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            text.push(self.advance());
            while self.peek().is_ascii_digit() {
                text.push(self.advance());
            }
        }

        // scientific notation!
        if self.peek() == 'e' || self.peek() == 'E' {
            text.push(self.advance());

            if self.peek() == '-' || self.peek() == '+' {   // allow for 5e-10 or 5e+10
                text.push(self.advance());
            }
            while self.peek().is_ascii_digit() {
                text.push(self.advance());
            }
        }

        let val: f64 = text.parse().unwrap_or(0.0);
        Token::Number(val)
    }

    fn string(&mut self, quote_type: char) -> Token {
        let mut text = String::new();

        while self.peek() != quote_type && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            text.push(self.advance());
        }

        if self.is_at_end() {

            return Token::String(text);
        }

        self.advance();
        
        Token::String(text)
    }
}