use crate::Error;
use crate::{
    parser::expr::{Expr, LiteralOp},
    Literal, Token, TokenType,
};
use std::sync::atomic::{AtomicUsize, Ordering};

struct Parser {
    tokens: Vec<Token>,
    current: AtomicUsize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: AtomicUsize::new(0),
        }
    }

    fn expression(&self) -> Result<Box<Expr>, Error> {
        self.equality()
    }

    fn equality(&self) -> Result<Box<Expr>, Error> {
        let mut expr = self.comparison()?;

        while self.find(&[&TokenType::BangEqual, &TokenType::EqualEqual]) {
            let operator = self.previous()?;
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary(expr, operator, right))
        }

        Ok(expr)
    }

    fn find(&self, tokens: &[&TokenType]) -> bool {
        for ttype in tokens {
            if self.check(**ttype) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        if let Ok(t) = self.peek() {
            return t.token_type == token;
        }
        false
    }

    fn advance(&self) -> Result<&Token, Error> {
        if !self.is_at_end() {
            self.current.fetch_add(1, Ordering::AcqRel);
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        if let Ok(t) = self.peek() {
            return t.token_type == TokenType::Eof;
        }
        false
    }

    fn peek(&self) -> Result<&Token, Error> {
        let current = self.current.load(Ordering::Relaxed);
        self.tokens
            .get(current)
            .ok_or_else(|| Error::ParsingError(format!("Could not get token at idx {}", current)))
    }

    fn previous(&self) -> Result<&Token, Error> {
        let current = self.current.load(Ordering::Relaxed);
        self.tokens.get(current - 1).ok_or_else(|| {
            Error::ParsingError(format!("Could not get token at idx {}", current - 1))
        })
    }

    fn comparison(&self) -> Result<Box<Expr>, Error> {
        let mut expr = self.term()?;

        while self.find(&[
            &TokenType::Greater,
            &TokenType::GreaterEqual,
            &TokenType::Less,
            &TokenType::LessEqual,
        ]) {
            let operator = self.previous()?;
            let right = self.term()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    fn term(&self) -> Result<Box<Expr>, Error> {
        let mut expr = self.factor()?;

        while self.find(&[&TokenType::Minus, &TokenType::Plus]) {
            let operator = self.previous()?;
            let right = self.factor()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    fn factor(&self) -> Result<Box<Expr>, Error> {
        let mut expr = self.unary()?;

        while self.find(&[&TokenType::Slash, &TokenType::Star]) {
            let operator = self.previous().unwrap();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    fn unary(&self) -> Result<Box<Expr>, Error> {
        while self.find(&[&TokenType::Bang, &TokenType::Minus]) {
            let operator = self.previous().unwrap();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary(operator, right)));
        }

        self.primary()
    }

    fn primary(&self) -> Result<Box<Expr>, Error> {
        if self.find(&[&TokenType::False]) {
            return Ok(Box::new(Expr::Literal(LiteralOp::False)));
        }
        if self.find(&[&TokenType::True]) {
            return Ok(Box::new(Expr::Literal(LiteralOp::True)));
        }
        if self.find(&[&TokenType::Nil]) {
            return Ok(Box::new(Expr::Literal(LiteralOp::Nil)));
        }

        if self.find(&[&TokenType::Number, &TokenType::String]) {
            let exp = match &self.previous()?.literal {
                Some(Literal::Str(s)) => Ok(Box::new(Expr::Literal(LiteralOp::Str(s.to_string())))),
                Some(Literal::Number(f)) => Ok(Box::new(Expr::Literal(LiteralOp::Number(*f)))),
                _ => Err(Error::ParsingError(format!(
                    "Invalid literal at token {}",
                    self.current.load(Ordering::Relaxed)
                ))),
            };
            return exp;
        }

        if self.find(&[&TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen)?;
            return Ok(Box::new(Expr::Grouping(expr)));
        }

        Err(Error::SyntaxError(
            String::from("TODO"),
            String::from("TODO"),
            String::from("TODO"),
        ))
    }

    fn consume(&self, ttype: TokenType) -> Result<(), Error> {
        Ok(())
    }
}
