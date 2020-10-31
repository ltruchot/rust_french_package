#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod exceptions;
mod graphemes;

/**
 * Takes any French word in the singular and return it in the plural
 *  @see https://fr.wiktionary.org/wiki/Annexe:Pluriels_irr%C3%A9guliers_en_fran%C3%A7ais
 */
#[must_use]
pub fn pluralize_word(word: &str) -> String {
    // TODO: composed words
    // ex: œil-de-bœuf -> œils-de-bœufs

    // TODO: disambiguation
    // "lieu" the place takes an "x", when "lieu" the fish take an "s"
    // "travail" the work takes an "x", when "travail" the horse tool take an "s"
    // "aïeul" the general ancestor pluralize to "aïeux", when "aïeul" direct grand-parent pluralize to "aïeuls"
    // "œil" the eye pluralize to "yeux", when "œil" the needle's eye pluralize to "œils"
    // "ciel" the sky pluralize to "cieux", when "ciel" the bed part pluralize to "ciels"
    // "banal" the ordinary pluralize to "banals", when "banal" the publication pluralize to "banaux"

    // TODO: two possibilities
    // final -> finals / finaux
    // ail -> ails / aulx

    // 1) global exceptions
    if word == "" {
        return String::new();
    }
    if word == "oeil" {
        return String::from("yeux");
    }
    if word == "viel" {
        return String::from("vieux");
    }
    if word == "topos" {
        return String::from("topo\u{ef}"); // topoï
    }

    // 2) local exceptions
    let result = String::from(word);

    // -- ending with "s", "x", "z"
    let last = graphemes::take_last(word);
    if last == "s" || last == "x" || last == "z" {
        return result;
    }

    // -- ending with "au", "eu", "ou", "al", "œu"
    let last_2_graphemes = graphemes::take_lasts(word, 2);
    let last2 = last_2_graphemes.as_str();

    if last2 == "au" || last2 == "eu" || last2 == "ou" || last2 == "al" || last2 == "\u{153}u" {
        return match last2 {
            // exceptions
            "ou" => {
                if exceptions::OU.contains(&word) {
                    result + "x"
                } else {
                    result + "s"
                }
            }
            "eu" | "au" | "\u{153}u" => {
                if exceptions::AU.contains(&word) || exceptions::EU.contains(&word) {
                    result + "s"
                } else {
                    result + "x"
                }
            }
            "al" => {
                if exceptions::AL.contains(&word) {
                    result + "s"
                } else {
                    graphemes::drop_last(word) + "ux"
                }
            }
            // not reachable
            _ => result,
        };
    }

    // -- ending with "ail"
    let last_3_graphemes = graphemes::take_lasts(word, 3);
    let last3 = last_3_graphemes.as_str();
    if last3 == "ail" && exceptions::AIL.contains(&word) {
        return graphemes::drop_lasts(word, 2) + "ux";
    }
    // 3) no exception: the most classic form
    result + "s"
}

#[cfg(test)]
#[allow(clippy::non_ascii_literal)]
mod tests {
    use super::pluralize_word;
    #[test]
    fn pluralize_works() {
        assert_eq!(pluralize_word(""), "");
        assert_eq!(pluralize_word("a"), "as");
        assert_eq!(pluralize_word("oeil"), "yeux");
        assert_eq!(pluralize_word("tests"), "tests");
        assert_eq!(pluralize_word("houx"), "houx");
        assert_eq!(pluralize_word("nez"), "nez");
        assert_eq!(pluralize_word("bleu"), "bleus");
        assert_eq!(pluralize_word("vieu"), "vieux");
        assert_eq!(pluralize_word("vœu"), "vœux");
        assert_eq!(pluralize_word("bateau"), "bateaux");
        assert_eq!(pluralize_word("landau"), "landaus");
        assert_eq!(pluralize_word("bijou"), "bijoux");
        assert_eq!(pluralize_word("matou"), "matous");
        assert_eq!(pluralize_word("animal"), "animaux");
        assert_eq!(pluralize_word("festival"), "festivals");
        assert_eq!(pluralize_word("corail"), "coraux");
        assert_eq!(pluralize_word("émail"), "émaux");
        assert_eq!(pluralize_word("chandail"), "chandails");
        assert_eq!(pluralize_word("voiture"), "voitures");
        assert_eq!(pluralize_word("vélo"), "vélos");
    }
}
