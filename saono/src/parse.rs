use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "saono.pest"]
pub struct Kers;