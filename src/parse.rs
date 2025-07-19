use crate::{
    command::{Command, GroupKind},
    tokens::{Symbol, Token},
};

#[derive(Debug)]
pub enum Expr<'a> {
    Literal(&'a str),
    Grouping(Box<Expr<'a>>),
    Combination(GroupKind, Box<Expr<'a>>, Box<Expr<'a>>),
}

impl Expr<'_> {
    pub fn produce(&self) -> Command {
        match self {
            Self::Literal(literal) => Command::Named(literal),
            Self::Grouping(inner) => inner.produce(),
            Self::Combination(kind, left, right) => kind.group([left.produce(), right.produce()]),
        }
    }
}

pub struct Parser<'a> {
    curr: usize,
    tokens: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { curr: 0, tokens }
    }

    pub fn expression(&mut self) -> Expr<'a> {
        self.sequence()
    }

    fn sequence(&mut self) -> Expr<'a> {
        let mut left = self.parallel();

        while self.matches(Symbol::Plus) {
            let right = self.parallel();
            left = Expr::Combination(GroupKind::Sequential, Box::new(left), Box::new(right));
        }
        left
    }

    fn parallel(&mut self) -> Expr<'a> {
        let mut left = self.primary();
        while self.matches(Symbol::And) {
            let right = self.primary();
            left = Expr::Combination(GroupKind::Parallel, Box::new(left), Box::new(right));
        }
        left
    }

    fn primary(&mut self) -> Expr<'a> {
        if self.matches(Symbol::OpenParen) {
            let expr = self.expression();
            if !self.matches(Symbol::CloseParen) {
                panic!("YOU DIDN'T CLOSE THE PARENTHESIS");
            }
            return Expr::Grouping(Box::new(expr));
        }
        let Token::Literal(literal) = self.advance() else {
            panic!("I don't know what to do with this!");
        };
        Expr::Literal(literal)
    }

    fn matches(&mut self, symbol: Symbol) -> bool {
        if self.check(symbol) {
            self.advance();
            return true;
        }
        false
    }

    fn check(&mut self, symbol: Symbol) -> bool {
        if self.at_end() {
            return false;
        }
        Token::Symbol(symbol) == self.tokens[self.curr]
    }

    fn at_end(&self) -> bool {
        self.curr >= self.tokens.len()
    }

    fn advance(&mut self) -> &Token<'a> {
        let expr = &self.tokens[self.curr];
        if !self.at_end() {
            self.curr += 1;
        }
        expr
    }
}
