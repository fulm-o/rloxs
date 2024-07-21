use super::token::LiteralKind;

#[derive(Debug)]
pub enum Expr {
    Assign{ name: String, expr: Box<Expr>},
    Literal { kind: LiteralKind},
    Variable(String),
    BinaryOp { left: Box<Expr>, operator: Operator, right: Box<Expr>},
    UnaryOp { operator: Operator, operand: Box<Expr>},
    Grouping( Box<Expr>),
}

#[derive(Debug)]
pub struct Operator {
    pub op_kind: OperatorKind,
    pub pos: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum OperatorKind {
    // Arithmetic operators
    ///"+"
    Add,
    ///"-"
    Subtract,
    ///"*"
    Multiply,
    ///"/"
    Divide,

    // Comparison operators
    ///"=="
    Equal,
    ///"!="
    NotEqual,
    ///">"
    Greater,
    ///">="
    GreaterEqual,
    ///"<"
    Less,
    ///"<="
    LessEqual,

    // Logical operators
    ///"and"
    And,
    ///"or"
    Or,

    // Unary operators
    ///"!"
    Not,
}