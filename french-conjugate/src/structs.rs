use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::types::PartialConjugation;

pub struct Verb {
    id: String,
    infinitive: String,
    data: PartialConjugation,
}

pub fn build_verb(id: &str, infinitive: &str, data: PartialConjugation) -> Verb {
    Verb {
        id: String::from(id),
        infinitive: String::from(infinitive),
        data,
    }
}
