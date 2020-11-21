use std::collections::HashMap;
use crate::parse::*;

pub struct TokenTable<'a> {
    seq: HashMap<String, Vec<Rule>>,
    out: Option<&'a TokenTable<'a>>,
}

pub struct Token {}

pub struct Subject {

}
