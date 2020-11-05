use crate::enums::{Person, Tense};

pub type Conjugation = (Tense, Person, String);
pub type PartialConjugation = Vec<Conjugation>;
pub type Persons = [String; 6];
