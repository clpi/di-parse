pub trait Entity {}
pub trait Action {}

pub trait Subject: Entity {}
pub trait Object: Entity {}
pub trait Verb: Action {}

pub trait Modal {}

pub struct Is<S: Entity, O: Entity> {
    subj: S,
    obj: O
}

pub struct Can<S: Entity, V: Verb> {
    subj: S,
    verb: V,
}

pub struct Has<S: Subject, O: Object> {
    subj: S,
    obj: O,
}

#[derive(Debug)]
pub enum Tense {
    Present,
    Past,
    Future,
}

#[derive(Debug)]
pub struct State<S: Modal> {
    state: S,
    tense: Tense,
}


