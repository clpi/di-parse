pub trait Entity {}
pub trait Action {}

pub trait Subject: Entity {}
pub trait Object: Entity {}
pub trait Verb: Action {}

pub trait Modal {}

#[derive(Debug, Clone)]
pub struct Is<S: Entity, O: Entity> {
    pub subj: S,
    pub obj: O
}

#[derive(Debug, Clone, Default)]
pub struct Can<S: Entity, V: Verb> {
    pub subj: S,
    pub verb: V,
}

#[derive(Debug, Clone, Default)]
pub struct Has<S: Subject, O: Object> {
    pub subj: S,
    pub obj: O,
}

#[derive(Debug, Clone)]
pub enum Tense {
    Present,
    Past,
    Future,
}

impl Default for Tense {
    fn default() -> Self {
        Tense::Present
    }
}

#[derive(Debug, Clone, Default)]
pub struct State<S: Modal> {
    pub state: S,
    pub tense: Tense,
}


