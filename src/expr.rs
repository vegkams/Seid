pub enum Expr { 
    Literal(Literal),
    Unary(UnaryOp),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Grouping(Box<Expr>),
}

pub enum UnaryOpType {
    Bang,
    Minus,
}

pub struct UnaryOp {
    pub op_type: UnaryOpType,
}

pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

pub enum BinaryOpType {
    Plus,
    Minus,
    Slash,
    Star,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

pub struct BinaryOp {
    pub op_type: BinaryOpType,
}