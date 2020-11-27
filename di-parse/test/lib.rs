pub use crate::run;

pub fn run_str(input: String) -> Result<()> {
    run(Some(input), None);
}

/// Simply tests that the parser can get through a few simple sentences without panicking
#[test]
pub fn multiple_sentences_only_commma_period_ok() {
    let input: String = " There were three people, one of whom was my friend. I am a bear. They were laughing, but I am crying. There are a good number of people at the party. There are a few people who went to the bar in Seattle for my BIRTHDAY"
        .to_string();
    dbg!(run_str(input).unwrap())

}

/// Simple test to ensure that the parser can make out text between parenthesis
#[test]
pub fn handles_text_between_parens_ok() {
    let input: String = "This is a (test of the) capabilities of the parser.".to_string();
}
