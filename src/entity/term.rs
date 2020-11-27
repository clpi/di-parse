pub enum Term {
    Subject(EntityRefKind),
    Object(EntityRefKind),
    Verb(VerbKind),
    Conjunction(Conjunction),
    Preposition(Preposition),
    Adverb(Adverb),
    Adjective(Adjective),
    Noun(Noun),
    Determinant(Determinant),
}

pub enum EntityRefKind {

}

pub enum VerbKind {
    PresentTense,
    PresentParticiple,
    PastTense,
    PastParticiple,
    FutureTense,
    FutureParticiple,
}

pub enum Conjunction {
    And,
    Or,
    Not,
    If,
}

pub enum Preposition {

}

pub enum Adverb {

}

pub enum Adjective {

}

pub enum Noun {

}

pub enum Determinant {
    The,
    AOrAn,

}
