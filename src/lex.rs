use crate::parse::*;
use pest::{
    iterators::{Pair, Pairs},
    RuleType,
};

pub struct Lexer {
    rule: Rule,
}

#[derive(Default, Debug)]
pub struct Statement {
    subj: Vec<String>,
    obj: Option<Vec<String>>,
    adj: Option<Vec<String>>,
    adv: Option<Vec<String>>,
    verb: Vec<String>,
}

impl Statement {

    pub fn with_subuj(subj: String) -> Self {
        Self { subj: vec![subj], ..Default::default() }
    }

    pub fn with_subj_verb_obj(
        subj: String,
        verb: String,
        obj: String
    ) -> Self {
        Self {
            subj: vec![subj],
            obj: Some(vec![obj]),
            verb: vec![verb], ..Default::default()
        }
    }
}


