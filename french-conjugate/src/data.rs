use crate::enums::{Person, Tense};
use crate::structs::{build_verb, Verb};

use lazy_static::lazy_static;

pub const ELER: [&str; 17] = [
    "celer",
    "geler",
    "ciseler",
    "harceler",
    "congeler",
    "marteler",
    "déceler",
    "modeler",
    "dégeler",
    "peler",
    "démanteler",
    "receler",
    "désurgeler",
    "regeler",
    "écarteler",
    "surgeler",
    "s’encasteler",
];

lazy_static! {
    static ref VERBS_3G: [Verb; 3] = [
        build_verb(
            "être#1",
            "être",
            vec![
                (
                    Tense::PresentIndicative,
                    Person::First,
                    String::from("suis")
                ),
                (Tense::PresentIndicative, Person::Second, String::from("es")),
                (Tense::PresentIndicative, Person::Third, String::from("est")),
                (
                    Tense::PresentIndicative,
                    Person::Fourth,
                    String::from("sommes")
                ),
                (
                    Tense::PresentIndicative,
                    Person::Fifth,
                    String::from("êtes")
                ),
                (
                    Tense::PresentIndicative,
                    Person::Sixth,
                    String::from("sont")
                ),
            ],
        ),
        build_verb(
            "avoir#1",
            "avoir",
            vec![
                (Tense::PresentIndicative, Person::First, String::from("ai")),
                (Tense::PresentIndicative, Person::Second, String::from("as")),
                (Tense::PresentIndicative, Person::Third, String::from("a")),
                (
                    Tense::PresentIndicative,
                    Person::Fourth,
                    String::from("avons")
                ),
                (
                    Tense::PresentIndicative,
                    Person::Fifth,
                    String::from("avez")
                ),
                (Tense::PresentIndicative, Person::Sixth, String::from("ont")),
            ],
        ),
        build_verb(
            "aller#1",
            "aller",
            vec![
                (
                    Tense::PresentIndicative,
                    Person::First,
                    String::from("vais")
                ),
                (
                    Tense::PresentIndicative,
                    Person::Second,
                    String::from("vas")
                ),
                (Tense::PresentIndicative, Person::Third, String::from("va")),
                (
                    Tense::PresentIndicative,
                    Person::Sixth,
                    String::from("vont")
                ),
            ],
        ),
    ];
}
