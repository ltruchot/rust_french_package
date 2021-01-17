use french_pluralize::pluralize_word;
use french_words_generator::write_commonnouns_irregular_plurals;

fn main() {
    println!("{}", pluralize_word("voiture")); // voitures
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
    match write_commonnouns_irregular_plurals() {
        Ok(_) => println!("tout va bien"),
        Err(err) => println!("err {}", err),
    }
}
