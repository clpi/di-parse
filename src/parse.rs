pub mod ast;
pub mod grammar;

extern crate pest;
use pest::{
    Parser, ParseResult,
    iterators::Pair
};
use crate::dict::Dictionary;


#[derive(Parser)]
#[grammar = "parse/grammar/grammar.pest"]
pub struct DivParser;

impl DivParser {

    pub fn print_pairs(pair: &Pair<'_, Rule>, num: usize) -> () {
        let pairs = pair.clone().into_inner();
        if pairs.clone().count() > 0 {
            println!("{:ident$}{:#?}: {:?}",
                "", pair.as_rule(), pair.as_str(), ident = num);
            for inner in pairs {
                use Rule::*;
                match inner.as_rule() {
                    period | EOI => {
                        println!("END EXPR ----- \n");
                        continue
                    },
                    comma | semicolon | colon => {
                        println!("END SUBEXPR ---- \n")
                    },
                    _ => {},
                }
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
        let dict = Dictionary::from_json("assets/dictionary.json");
        println!("{:?}", serde_json::to_string(&dict));
        match words {
            Ok(pairs) => {
                for pair in pairs {
                    Self::print_pairs(&pair, 0);
                    match dict.get_def(pair.as_str()) {
                        Some(def) => println!("{} {:?}", pair.as_str(), def),
                        None => println!(""),
                    };
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
}
