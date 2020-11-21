extern crate pest;

use pest::{
    Parser, ParserState, ParseResult,
    iterators::{Pair, Pairs}
};
use pest_derive;

//#[grammar = "grammar.pest"]
#[derive(Parser)]
#[grammar = "grammar.pest"]
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
                    // println!("Rule:\t{:?}\nSpan:\t{:?}\nText:\t{:?}\n",
                    //     pair.as_rule(),pair.as_span(), pair.as_str());
                    // for inner in pair.into_inner() {
                    //     Self::match_pair(inner);
                    // }
                }
            },
            Err(e) => panic!("{}", e),
        }
        // println!("{:#?}", words);
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
}
