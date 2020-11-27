use pest::{
    Parser, ParseResult,
    iterators::Pair
};

#[derive(Debug, Clone)]
#[grammar = "parse/grammar/grammar.pest"]
pub struct Grammar {}
