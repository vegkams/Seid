use crate::Token;
use std::fmt;
/*
Expression grammar:

expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )*;
term           → factor ( ( "-" | "+" )  factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "-" | "!" ) unary | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
*/

pub enum Expr<'a> {
    Literal(LiteralOp),
    Unary(&'a Token, Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, &'a Token, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
}

impl fmt::Display for Expr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Literal(op) => write!(f, "{}", &op),
            Expr::Unary(t, exp) => {
                if let Ok(s) = std::str::from_utf8(&t.lexeme) {
                    return write!(f, "( {} {})", s, &*exp);
                } else {
                    return Err(fmt::Error);
                }
            }
            Expr::Binary(exp_lhs, t, exp_rhs) => {
                if let Ok(s) = std::str::from_utf8(&t.lexeme) {
                    return write!(f, "( {} {} {})", s, &*exp_lhs, &*exp_rhs);
                } else {
                    return Err(fmt::Error);
                }
            }
            Expr::Grouping(exp) => write!(f, "( group {})", &*exp),
        }
    }
}

pub enum LiteralOp {
    Number(f64),
    Str(String),
    True,
    False,
    Nil,
}

impl fmt::Display for LiteralOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralOp::Number(n) => write!(f, "{}", n),
            LiteralOp::Str(s) => write!(f, "{}", s),
            LiteralOp::True => write!(f, "True"),
            LiteralOp::False => write!(f, "False"),
            LiteralOp::Nil => write!(f, "Nil"),
        }
    }
}
