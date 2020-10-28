#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod graphemes;

#[must_use]
pub fn pluralize(s: &str) -> String {
    // global exception
    if s == "oeil" {
        return String::from("yeux");
    }

    let result = String::from(s);
    // ambiguous
    // lieu (prend un "x" si "endroit", prend un "s" si poisson)

    // les deux possibles
    // final / finaux

    // ending with s, x, z
    let last = graphemes::take_last(s);
    if last == "s" || last == "x" || last == "z" {
        return result;
    }

    // ending with au, eu, ou
    let lasts = graphemes::take_lasts(s, 2);
    if lasts == "au" || lasts == "eu" || lasts == "ou" || lasts == "al" {
        return match lasts.as_str() {
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
                _ => graphemes::drop_last(&result) + "ux",
            },
            // classic
            _ => result + "x",
        };
    }
    result + "s"
}

#[cfg(test)]
mod tests {
    use super::pluralize;
    #[test]
    fn pluralize_works() {
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
        assert_eq!(pluralize("voiture"), "voitures");
    }
}
