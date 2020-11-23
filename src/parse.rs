extern crate pest;
use pest::{
    Parser, ParseResult,
    iterators::Pair
};


#[derive(Parser, Debug)]
#[grammar = "grammar/grammar.pest"]
pub struct DivParser;

impl DivParser {

    pub fn print_pairs(pair: &Pair<'_, Rule>, num: usize) -> () {
        let pairs = pair.clone().into_inner();
        if pairs.clone().count() > 0 {
            println!("{:ident$}{:#?}: {:?}",
                "", pair.as_rule(), pair.as_str(), ident = num);
            for inner in pairs {
                Self::print_pairs(&inner, num + 2);
            }
        } else {
            println!("{:ident$}{:#?}: {}", "",
                pair.as_rule(),
                pair.as_span().as_str(),
                ident = num);
        }
    }

    pub fn get_pairs(input: String) -> ParseResult<()> {
        let words = Self::parse(Rule::full, input.as_str());
        match words {
            Ok(pairs) => {
                for pair in pairs {
                    Self::print_pairs(&pair, 0);
                    for inner in pair.into_inner() {
                        Self::match_pair(inner);
                    }
                }
            },
            Err(e) => panic!("{}", e),
        }
        Ok(())
    }

    pub fn match_pair(pair: Pair<'_, Rule>) -> () {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::alpha => println!("Alpha:\t{:?}", inner.as_str()),
                Rule::digit => println!("Digit:\t{:?}", inner.as_str()),
                Rule::pronoun => println!("Pronoun:\t{:?}", inner.as_str()),
                Rule::punc => println!("Punct:\t{:?}", inner.as_str()),
                Rule::ws => println!("Space:\t{:?}", inner.as_str()),
                _ => continue,
            }
        }
    }

    pub fn match_char(pair: Pair<'_, Rule>) -> () {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::alpha => print!("Alpha:\t{:?}\n", inner.as_str()),
                Rule::digit => print!("Digit:\t{:?}\n", inner.as_str()),
                Rule::punc => {
                    print!("Punct:\t{:?}\n", inner.as_str());
                    Self::match_punc(inner);
                },
                Rule::ws => print!("Space:\t{:?}\n", inner.as_str()),
                _ => continue,
            }
        }
    }

    pub fn match_punc(pair: Pair<'_, Rule>) -> () {
        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::period => print!("Period:\t{:?}\n", inner.as_str()),
                Rule::comma => print!("Comma:\t{:?}\n", inner.as_str()), // TODO handle both sides
                Rule::exclamation => print!("Exclamation:\t{:?}\n", inner.as_str()),
                Rule::question_mark => {
                    print!("Exclamation:\t{:?}\n", inner.as_str());
                },
                Rule::semicolon => print!("Semicolon:\t{:?}\n", inner.as_str()), //TODO handle both sides
                Rule::colon => print!("Colon:\t{:?}", inner.as_str()), //TODO handle both sides
                _ => continue,
            }
        }
    }

}
