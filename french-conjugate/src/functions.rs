use crate::data::ELER;
use crate::enums::{Person, Tense};
use crate::types::{PartialConjugation, Persons};
use grapheme_picker::{drop_last, drop_lasts, take_last, take_lasts};

fn get_radical(verb: &str) -> String {
    let last_4_graphemes = take_lasts(verb, 4);
    let last4 = last_4_graphemes.as_str();
    match last4 {
        "eler" => {
            if ELER.contains(&verb) {
                drop_lasts(verb, 4) + "\u{e8}le" // "èle" like in "gèle"
            } else {
                drop_lasts(verb, 2) + "le"
            }
        }
        _ => drop_last(verb),
    }
}

fn get_conjugation(verb: &str, tense_person: (&Tense, Person)) -> String {
    let last_2_graphemes = take_lasts(verb, 2);
    let last2 = last_2_graphemes.as_str();
    let radical = get_radical(verb);
    match tense_person {
        (Tense::PresentIndicative, Person::First) => match last2 {
            "er" => radical,
            "ir" => radical + "s",
            _ => String::from(verb), // should not append
        },

        (Tense::PresentIndicative, Person::Second) => match last2 {
            "er" | "ir" => radical + "s",
            _ => String::from(verb), // should not append
        },

        (Tense::PresentIndicative, Person::Third) => match last2 {
            "er" => radical,
            "ir" => radical + "t",
            _ => String::from(verb), // should not append
        },
        (Tense::PresentIndicative, Person::Fourth) => match last2 {
            "er" => {
                let root = drop_last(&radical);
                let third_last = take_last(&root);
                if third_last == "g" {
                    // ex: mangeons
                    root + "eons"
                } else if third_last == "c" {
                    // ex: lançons
                    drop_last(&root) + "\u{e7}ons"
                } else {
                    // ex: marchons
                    root + "ons"
                }
            }
            "ir" => radical + "ssons",
            _ => String::from(verb), // should not append
        },
        (Tense::PresentIndicative, Person::Fifth) => match last2 {
            "er" => radical + "z",
            "ir" => radical + "ssez",
            _ => String::from(verb), // should not append
        },
        (Tense::PresentIndicative, Person::Sixth) => match last2 {
            "er" => radical + "nt",
            "ir" => radical + "ssent",
            _ => String::from(verb), // should not append
        },
    }
}

fn get_exception_or_conj(
    verb: &str,
    info: (&Tense, Person),
    partial_conjugation: &PartialConjugation,
) -> String {
    let (tense, person) = info;
    partial_conjugation
        .iter()
        .find(|t| &t.0 == tense && t.1 == person)
        .map_or_else(
            || get_conjugation(verb, (tense, person)),
            |conj| String::from(&conj.2),
        )
}

fn get_tense(verb: &str, tense: &Tense, opt_conjugation: Option<PartialConjugation>) -> Persons {
    let partial_conjugation = opt_conjugation.unwrap_or_default();
    [
        get_exception_or_conj(verb, (tense, Person::First), &partial_conjugation),
        get_exception_or_conj(verb, (tense, Person::Second), &partial_conjugation),
        get_exception_or_conj(verb, (tense, Person::Third), &partial_conjugation),
        get_exception_or_conj(verb, (tense, Person::Fourth), &partial_conjugation),
        get_exception_or_conj(verb, (tense, Person::Fifth), &partial_conjugation),
        get_exception_or_conj(verb, (tense, Person::Sixth), &partial_conjugation),
    ]
}

#[cfg(test)]
#[allow(clippy::non_ascii_literal)]
mod tests {
    use super::{get_conjugation, get_exception_or_conj, get_tense};
    use super::{Person, Tense};
    #[test]
    fn get_conjugation_works() {
        assert_eq!(
            get_conjugation("manger", (&Tense::PresentIndicative, Person::First)),
            String::from("mange")
        );
        assert_eq!(
            get_conjugation("manger", (&Tense::PresentIndicative, Person::Second)),
            String::from("manges")
        );
        assert_eq!(
            get_conjugation("manger", (&Tense::PresentIndicative, Person::Third)),
            String::from("mange")
        );
        assert_eq!(
            get_conjugation("manger", (&Tense::PresentIndicative, Person::Fourth)),
            String::from("mangeons")
        );
        assert_eq!(
            get_conjugation("manger", (&Tense::PresentIndicative, Person::Fifth)),
            String::from("mangez")
        );
        assert_eq!(
            get_conjugation("manger", (&Tense::PresentIndicative, Person::Sixth)),
            String::from("mangent")
        );
        assert_eq!(
            get_conjugation("marcher", (&Tense::PresentIndicative, Person::Fourth)),
            String::from("marchons")
        );
        assert_eq!(
            get_conjugation("lancer", (&Tense::PresentIndicative, Person::Fourth)),
            String::from("lançons")
        );
        assert_eq!(
            get_conjugation("appeler", (&Tense::PresentIndicative, Person::First)),
            String::from("appelle")
        );
        assert_eq!(
            get_conjugation("déceler", (&Tense::PresentIndicative, Person::First)),
            String::from("décèle")
        );
        assert_eq!(
            get_conjugation("finir", (&Tense::PresentIndicative, Person::First)),
            String::from("finis")
        );
        assert_eq!(
            get_conjugation("finir", (&Tense::PresentIndicative, Person::Second)),
            String::from("finis")
        );
        assert_eq!(
            get_conjugation("finir", (&Tense::PresentIndicative, Person::Third)),
            String::from("finit")
        );
        assert_eq!(
            get_conjugation("finir", (&Tense::PresentIndicative, Person::Fourth)),
            String::from("finissons")
        );
        assert_eq!(
            get_conjugation("finir", (&Tense::PresentIndicative, Person::Fifth)),
            String::from("finissez")
        );
        assert_eq!(
            get_conjugation("finir", (&Tense::PresentIndicative, Person::Sixth)),
            String::from("finissent")
        );
    }

    #[test]
    fn get_exception_or_conj_works() {
        assert_eq!(
            get_exception_or_conj(
                "manger",
                (&Tense::PresentIndicative, Person::First),
                &vec![]
            ),
            String::from("mange")
        );
        assert_eq!(
            get_exception_or_conj(
                "aller",
                (&Tense::PresentIndicative, Person::First),
                &vec![(
                    Tense::PresentIndicative,
                    Person::First,
                    String::from("vais")
                )]
            ),
            String::from("vais")
        );
    }

    #[test]
    fn get_tense_works() {
        assert_eq!(
            get_tense(
                "aller",
                &Tense::PresentIndicative,
                Some(vec![
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
                ]),
            ),
            [
                String::from("vais"),
                String::from("vas"),
                String::from("va"),
                String::from("allons"),
                String::from("allez"),
                String::from("vont")
            ]
        );
    }
}
