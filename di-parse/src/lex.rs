use crate::parse::*;
use pest::{
    iterators::{Pair, Pairs},
    RuleType,
};

#[derive(Clone, Debug)]
pub struct Lexer {
    pub rule: Rule,
}

#[derive(Default, Debug, Clone)]
pub struct Statement {
    pub subj: Vec<String>,
    pub obj: Option<Vec<String>>,
    pub adj: Option<Vec<String>>,
    pub adv: Option<Vec<String>>,
    pub verb: Vec<String>,
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


