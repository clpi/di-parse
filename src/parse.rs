// heuristic h1: If the head has an is-for feature, and a. the modifier matches one of the valuees of the is-for feature, then relation holds, or b. any of the hypernyms of the modifier match any of the values of the is-for feature, then holds *probably*
// heuristic h1: if head has a location-of feature and if any values of the location-of feature have HAS-SUBJECT or HAS-OBJECT features and:
//      a. the modifier matches any of the values of hte has-subject or has-object features then the relationship almost certainly hodlds
//      b. any of the hypernyms of the modifier match any of the has-subject or has-object features, then the relationship probably holds
// heuristic h3: if the headnoun has a HASOBJECT feature and a. the modifier matches any of the has-object values, then the relationship porbably holds, or b. any of the hypernyms of the modifier matches any of hte has-object values, the relationship might hold
extern crate pest;
use pest::{
    Parser, ParseResult,
    iterators::Pair
};


#[derive(Parser)]
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
}
