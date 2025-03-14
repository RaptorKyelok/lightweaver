
use super::tokens::{Token, TokenType::{self, *}};
use std::{collections::HashMap, string::String};

type Keywords = HashMap<String, TokenType>;

pub struct ScannerError {}



pub struct Scanner {

    start:   usize, // start of the current lexeme
    current: usize, // current character
    line:    usize,

    source: Vec<char>,
    tokens: Vec<Token>,

    keywords: Keywords
}

impl Scanner {
    pub fn new(source: &str) -> Self {

        let bytes = source.chars().collect();

        Self {
            start:   0,
            current: 0,
            line:    1,

            source: bytes,
            tokens: Vec::new(),

            keywords: get_keywords()
        }
    }

    pub fn scan_tokens(source: &str) -> Vec<Token> {
        let mut scanner = Scanner::new(source);

        while !scanner.is_eof() {
            scanner.start = scanner.current;

            scanner.scan_token();
        }

        scanner.finalize();

        scanner.tokens
    }

    // #region Tokenizing functions

    fn scan_token(&mut self) {
        let ch     = self.advance();

        match ch {

            ' '  => (),
            '\r' => (),
            '\t' => (),

            '='  => self.add_token("=", Equals),
            ':'  => self.add_token(":", Colon),
            ';'  => self.add_token(";", SemiColon),
            '{'  => self.add_token("{", LeftCurly),
            '}'  => self.add_token("}", RightCurly),
            '"'  => self.scan_string(),

            '-' => {
                if self.match_('>') {
                    self.advance();
                    self.add_token("->", RightThinArrow);
                }
                else {
                    self.error(&format!("Unknown character {}", ch));
                }
            }

            '\n' => {
                self.line += 1;
            }

            _ => {

                if is_digit(ch) {
                    self.scan_number();
                }
                else if is_alpha(ch) {
                    self.scan_identifier();
                }
                else {
                    self.error(&format!("Unknown character '{}'", ch));
                }

            },
        }
    }

    fn scan_number(&mut self) {
        
        while is_alpha(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek()) {
            // consume the '.'
            self.advance();

            while is_alpha(self.peek()) {
                self.advance();
            }
        }

        self.add_token(&self.get_lexeme(), Number);
    }

    fn scan_string(&mut self) {

        let mut bytes: Vec<char> = vec![];

        while self.peek() != '"' && !self.is_eof() {

            if self.peek() == '\n' {
                self.line += 1;
            }

            else if self.peek() == '\\' && self.has_next() {
                self.advance();
                
                bytes.push(self.scan_escape());
            }

            bytes.push(self.advance());
        }

        if self.is_eof() {
            self.error("Unterminated string");
        }

        // the closing "
        self.advance();

        self.add_token(&chars_to_str(&bytes), TokenType::String);

    }

    fn scan_escape(&mut self) -> char {
        let ch = self.advance();

        match ch {
            '\"' |
            '\'' |
            '\\' => ch,
            'n'  => '\n',
            't'  => '\t',

            _   => {
                self.error(&format!("Unknown escape character: '{}'", ch));
                '\0'
            }
        }
    }

    fn scan_identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let lexeme = self.get_lexeme();

        let type_ = self.keywords.get(&lexeme);
        let type_ = type_.unwrap_or(&Identifier);

        self.add_token(&lexeme, type_.clone());
    }

    // #endregion
    // #region scanner functions

    fn is_eof(&self) -> bool {
        !self.has_next()
    }

    fn has_next(&self) -> bool {
        self.current < self.source.len()
    }

    fn add_token(&mut self, lexeme: &str, type_: TokenType) {
        self.tokens.push(Token::new(type_, &lexeme, self.line));
    }

    fn advance(&mut self) -> char {
        let i = self.current;
        self.current += 1;

        self.source[i]
    }

    fn seek(&self, index: usize) -> char {
        if self.current + index >= self.source.len() {
            return '\0';
        }

        let i = self.current + index;
        self.source[i]
    }

    fn peek(&self) -> char {
        self.seek(0)
    }

    fn peek_next(&self) -> char {
        self.seek(1)
    }

    fn match_(&self, ch: char) -> bool {
        ch == self.peek()
    }

    fn match_number(&self) -> bool {
        let ch = self.peek();

        is_digit(ch)
    }

    fn get_lexeme(&self) -> String {
        chars_to_str(&self.source[self.start..self.current])
    }
    
    fn finalize(&mut self) {
        self.tokens.push(Token::new(EOF, "", self.line));
    }

    fn error(&mut self, msg: &str) {
        panic!("{}", msg)
    }

    // #endregion
    
}


fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_alpha(ch: char) -> bool {
       ('a' <= ch && ch <= 'z')
    || ('A' <= ch && ch <= 'Z')
    ||  ch == '_'
    
}

fn is_alpha_numeric(ch: char) -> bool {
    is_alpha(ch) || is_digit(ch)
}

fn chars_to_str(chars: &[char]) -> String {
    chars.iter().cloned().collect::<String>()
}

pub fn get_keywords() -> HashMap<String, TokenType> {
    let mut dict = HashMap::new();

    let mut add = |k: &str, v: TokenType| {
        dict.insert(String::from(k), v);
    };

    add("let",  Let);
    add("Rect", Rect);

    dict
}