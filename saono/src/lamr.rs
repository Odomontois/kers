// use num::BigInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    App { fun: Box<Expr>, arg: Box<Expr> },
    Lam { size: usize, body: Box<Expr> },
    Rec(Vec<Expr>),
    Concat { left: Box<Expr>, right: Box<Expr> },
    Get(usize),
    Num(u64),
}
