#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod graphemes;

#[must_use]
pub fn pluralize(s: &str) -> String {
    // TODO: disambiguation
    // "lieu" the place take an "x", when "lieu" the fish take an "s"
    // "travail" the work take an "x", "travail" the horse tool take an "s"

    // TODO: two possibilities
    // final -> finals / finaux
    // ail -> ails / aulx

    // 1) global exceptions
    if s == "" {
        return String::new();
    }
    if s == "oeil" {
        return String::from("yeux");
    }

    // 2) local exceptions
    let result = String::from(s);

    // -- ending with "s", "x", "z"
    let last = graphemes::take_last(s);
    if last == "s" || last == "x" || last == "z" {
        return result;
    }

    // -- ending with "au", "eu", "ou", "al"
    let last_2_graphemes = graphemes::take_lasts(s, 2);
    let last2 = last_2_graphemes.as_str();
    if last2 == "au" || last2 == "eu" || last2 == "ou" || last2 == "al" {
        return match last2 {
            // exceptions
            "ou" => match s {
                "bijou" | "chou" | "genou" | "caillou" | "hibou" | "joujou" | "pou" | "ripou"
                | "chouchou" | "boutchou" => result + "x",
                _ => result + "s",
            },
            "eu" | "au" => match s {
                "bleu" | "\u{e9}meu" | "pneu" | "enfeu" | "landau" | "sarrau" | "grau" | "unau"
                | "senau" => result + "s",
                _ => result + "x",
            },
            "al" => match s {
                "aval"
                | "bal"
                | "banal"
                | "bancal"
                | "cal"
                | "carnaval"
                | "c\u{e9}r\u{e9}monial"
                | "choral"
                | "\u{e9}tal"
                | "fatal"
                | "festival"
                | "natal"
                | "naval"
                | "pal"
                | "r\u{e9}cital"
                | "r\u{e9}gal"
                | "tonal"
                | "val"
                | "virginal" => result + "s",
                _ => graphemes::drop_last(s) + "ux",
            },
            // not reachable
            _ => result,
        };
    }

    // -- ending with "ail"
    let last_3_graphemes = graphemes::take_lasts(s, 3);
    let last3 = last_3_graphemes.as_str();
    if last3 == "ail" {
        return match s {
            "bail" | "corail" | "\u{e9}mail" | "gemmail" | "soupirail" | "travail" | "vantail"
            | "vitrail" | "fermail" => graphemes::drop_lasts(s, 2) + "ux",
            // not reachable
            _ => result + "s",
        };
    }
    // 3) no exception: the most classic form
    result + "s"
}

#[cfg(test)]
#[allow(clippy::non_ascii_literal)]
mod tests {
    use super::pluralize;
    #[test]
    fn pluralize_works() {
        assert_eq!(pluralize(""), "");
        assert_eq!(pluralize("a"), "as");
        assert_eq!(pluralize("oeil"), "yeux");
        assert_eq!(pluralize("tests"), "tests");
        assert_eq!(pluralize("animaux"), "animaux");
        assert_eq!(pluralize("nez"), "nez");
        assert_eq!(pluralize("bleu"), "bleus");
        assert_eq!(pluralize("vieu"), "vieux");
        assert_eq!(pluralize("bateau"), "bateaux");
        assert_eq!(pluralize("landau"), "landaus");
        assert_eq!(pluralize("bijou"), "bijoux");
        assert_eq!(pluralize("matou"), "matous");
        assert_eq!(pluralize("animal"), "animaux");
        assert_eq!(pluralize("festival"), "festivals");
        assert_eq!(pluralize("corail"), "coraux");
        assert_eq!(pluralize("émail"), "émaux");
        assert_eq!(pluralize("chandail"), "chandails");
        assert_eq!(pluralize("voiture"), "voitures");
        assert_eq!(pluralize("vélo"), "vélos");
    }
}
