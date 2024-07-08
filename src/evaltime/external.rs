#[derive(Debug, Clone, Copy)]
pub enum ExternalValue {
    IntegerOp(IntegerOperation),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub enum IntegerOperation {
    Plus,
    Minus,
    Times,
    Div,
    Mod,
    Pow,
    Neg,
    Abs,
    Signum,
}
