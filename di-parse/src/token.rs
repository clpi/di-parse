pub mod term;
pub mod noun;

use std::collections::HashMap;
use crate::parse::*;

pub struct TokenTable<'a> {
    seq: HashMap<String, Vec<Rule>>,
    out: Option<&'a TokenTable<'a>>,
}


pub struct Token {
    term: String,
    start_offs: usize,
    pos: usize,
}

pub struct Subject {

}

pub struct Rule {


}

pub struct Tokenizer {

}

impl Tokenizer {
}
