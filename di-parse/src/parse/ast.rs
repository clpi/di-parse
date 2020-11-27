use pest::iterators::{Pair, Pairs};
use crate::parse::*;

#[derive(Debug, Default)]
pub struct Ast<'a> {
    children: Vec<AstNode<'a>>,
}

#[derive(Debug)]
pub struct AstNode<'a> {
    word: &'a str,
    rule: Rule,
    children: Vec<AstNode<'a>>,
}

#[derive(Debug)]
pub struct AstSentenceNode {

}

#[derive(Debug)]
pub struct AstCharacterNode {

}

#[derive(Debug)]
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

impl<'a> From<Pair<'a, Rule>> for Ast<'a> {
    fn from(pair: Pair<'a, Rule>) -> Self {
        let mut children: Vec<AstNode<'a>> = Vec::new();
        for inner in pair.clone().into_inner() {
            children.push(AstNode::from(inner));
        }
        Self { children }
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
