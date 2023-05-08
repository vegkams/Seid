use std::fmt;
use crate::Token;

pub enum Expr { 
    Literal(LiteralOp),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Literal(op) => write!(f, "{}", &op.op_type),
            Expr::Unary(t, exp) =>  {
                if let Ok(s) = std::str::from_utf8(&t.lexeme) {
                    return write!(f, "( {} {})", s, &*exp);
                } else {
                    return Err(fmt::Error);
                }
                
            }
            Expr::Binary(exp1, t, exp2) => {
                if let Ok(s) = std::str::from_utf8(&t.lexeme) {
                    return write!(f, "( {} {} {})", s, &*exp1, &*exp2);
                } else {
                    return Err(fmt::Error);
                }
            }
            Expr::Grouping(exp) => write!(f, "( group {})", &*exp),
        }
    }
}


pub enum LiteralOpType {
    Number(f64),
    Str(String),
    True,
    False,
    Nil,
}


impl fmt::Display for LiteralOpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralOpType::Number(n) => write!(f, "{}", n),
            LiteralOpType::Str(s)    => write!(f, "{}", s),
            LiteralOpType::True      => write!(f, "True"),
            LiteralOpType::False     => write!(f, "False"),
            LiteralOpType::Nil       => write!(f, "Nil"),
        }
    }
}

pub struct LiteralOp {
    pub op_type: LiteralOpType,
}

