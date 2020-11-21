use pest::iterators::{Pair, Pairs};
use crate::parse::*;

pub struct Ast<'a> {
    children: Vec<AstNode<'a>>,
}

pub struct AstNode<'a> {
    word: &'a str,
    rule: Rule,
    children: Vec<AstNode<'a>>,
}

pub struct AstSentenceNode {

}

pub enum AstParagraphNode {

}

impl<'a> From<Pair<'a, Rule>> for AstNode<'a> {
    fn from(pair: Pair<'a, Rule>) -> Self {
        let mut children: Vec<AstNode<'a>> = Vec::new();
        for inner in pair.clone().into_inner() {
            children.push(AstNode::from(inner));
        }
        Self {
            word: pair.as_str(),
            rule: pair.as_rule(),
            children
        }
    }
}

impl<'a> From<Pairs<'a, Rule>> for Ast<'a> {
    fn from(pairs: Pairs<'a, Rule>) -> Self {
        let mut children: Vec<AstNode<'a>> = Vec::new();
        for pair in pairs {
            children.push(AstNode::from(pair));
        }
        Self { children }
    }
}
