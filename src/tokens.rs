#[derive(Debug, Clone, PartialEq)]
pub enum Token<'a> {
    Literal(&'a str),
    Symbol(Symbol),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    Plus,
    And,
    OpenParen,
    CloseParen,
}

impl Symbol {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Plus),
            '&' => Some(Self::And),
            '(' => Some(Self::OpenParen),
            ')' => Some(Self::CloseParen),
            _ => None,
        }
    }
}

pub struct Tokenizer<'a> {
    start: usize,
    curr: usize,
    source: &'a str,
    chars: Vec<char>,
    tokens: Vec<Token<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            start: 0,
            curr: 0,
            source,
            chars: source.chars().collect::<Vec<char>>(),
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self) -> &Vec<Token<'a>> {
        while self.start < self.source.len() {
            self.one_token();
            self.start = self.curr;
        }
        &self.tokens
    }

    fn one_token(&mut self) {
        let c = self.chars[self.curr];
        self.curr += 1;
        if let Some(symbol) = Symbol::from_char(c) {
            self.tokens.push(Token::Symbol(symbol));
            return;
        }
        // maybe we are looking at a comment, check for #.
        if c == '#' {
            // Discard everything.
            self.curr = self.source.len();
            return;
        }
        // Now we have to be looking at an identifier. Capture values that are not symbols.
        while self.curr < self.source.len() && Symbol::from_char(self.chars[self.curr]).is_none() {
            self.curr += 1;
        }
        // capture ident:
        let literal = &self.source[self.start..self.curr];
        if !literal.trim().is_empty() {
            self.tokens.push(Token::Literal(literal.trim()));
        }
    }
}
