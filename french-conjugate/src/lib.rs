#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
extern crate lazy_static;
use grapheme_picker::{drop_last, drop_lasts, take_last, take_lasts};

mod data;
mod enums;
mod functions;
mod structs;
mod types;

use enums::Person;

#[must_use]
pub fn conjugate_verb(verb: &str, person: Person) -> String {
    String::new()
}

#[cfg(test)]
#[allow(clippy::non_ascii_literal)]
mod tests {
    use super::conjugate_verb;
    use super::enums::Person;
    #[test]
    fn conjugate_verb_works() {
        assert_eq!(conjugate_verb("aller", Person::First), String::new())
    }
}
