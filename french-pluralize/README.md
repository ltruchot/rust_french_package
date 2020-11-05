# french-pluralize

A Rust library to help users to get plurals of French words in different ways.

It mainly follow the french plural rules available on the [French Wiktionary](https://fr.wiktionary.org/wiki/Annexe:Pluriels_irr%C3%A9guliers_en_fran%C3%A7ais)

```toml
[dependencies]
french-pluralize = "0.2.3"
```

```rust
use french-pluralize::pluralize_word;

println!("{}", pluralize_word("voiture")); // voitures

assert_eq!(pluralize_word("test"), "tests");
assert_eq!(pluralize_word("dos"), "dos");
assert_eq!(pluralize_word("oeil"), "yeux");
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
```