#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use mysql::prelude::*;
use mysql::*;

use std::fs::File;
use std::io::prelude::*;

mod structs;
use structs::{CommonNoun, Lemma};

use french_pluralize::pluralize_word;
use french_words::structs::NounGender;

/// # Errors
///
/// Will return `Err` if...
pub fn select_common_nouns() -> Result<Vec<CommonNoun>> {
    let url = "mysql://root:@localhost:3306/morphalou3";

    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let selected_nouns = conn.query_map(
        "SELECT
        lemma.content AS singular_content,
            lemma.gender AS gender,
            lemma.phonetic1 AS singular_phonetic,
            inflection.content AS plural_content,
            inflection.phonetic1 AS plural_phonetic,
            fem.fem_content AS feminine_content,
            fem.fem_phonetic AS feminine_phonetic,
            fem.fem_plural AS feminine_plural_content,
            fem.fem_plural_phonetic AS feminine_plural_phonetic
        FROM
            lemma
        LEFT JOIN inflection ON lemma.lemmaID = inflection.lemmaID AND inflection.number = 'plural'
        LEFT JOIN(
            SELECT
                fem.content AS fem_content,
                fem.lemmaID AS fem_id,
                feminineOf,
                inflection_fem.content AS fem_plural,
                fem.phonetic1 AS fem_phonetic,
                inflection_fem.phonetic1 AS fem_plural_phonetic
            FROM
                lemma AS fem
            LEFT JOIN inflection AS inflection_fem
            ON
                fem.lemmaID = inflection_fem.lemmaID AND inflection_fem.number = 'plural'
            WHERE
                fem.category = 'commonNoun' AND fem.feminineOf IS NOT NULL
        ) AS fem
        ON
            fem.feminineOf = lemma.lemmaID
        WHERE
            lemma.category = 'commonNoun' AND lemma.feminineOf IS NULL
        ORDER BY
            lemma.content",
        |(
            singular_content,
            gender,
            singular_phonetic,
            plural_content,
            plural_phonetic,
            feminine_content,
            feminine_phonetic,
            feminine_plural_content,
            feminine_plural_phonetic,
        ): (
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        )| {
            CommonNoun {
                gender,
                singular: Lemma {
                    content: singular_content,
                    phonetic: singular_phonetic,
                },
                plural: match plural_content {
                    Some(s) => Some(Lemma {
                        content: s,
                        phonetic: plural_phonetic,
                    }),
                    None => None,
                },
                feminine: match feminine_content {
                    Some(s) => Some(Lemma {
                        content: s,
                        phonetic: feminine_phonetic,
                    }),
                    None => None,
                },
                feminine_plural: match feminine_plural_content {
                    Some(s) => Some(Lemma {
                        content: s,
                        phonetic: feminine_plural_phonetic,
                    }),
                    None => None,
                },
            }
        },
    )?;
    // selected_nouns.sort_by(|a, b| b.singular.content.cmp(&a.singular.content));
    Ok(selected_nouns)
}

/// # Errors
///
/// Will return `Err` if...
pub fn write_commonnouns_irregular_plurals() -> Result<()> {
    let plain_common_nouns = select_common_nouns()?;
    let common_nouns = plain_common_nouns
        .iter()
        .filter_map(|noun| match &noun.plural {
            Some(plural_lemma) => {
                if pluralize_word(&noun.singular.content) == plural_lemma.content {
                    None
                } else {
                    Some(noun.get_as_str())
                }
            }
            None => None,
        })
        .collect::<Vec<String>>();
    let mut file = File::create("./french-words/src/common_nouns.rs")?;
    let content = [
        "use crate::structs::{StaticLemma, StaticCommonNoun, NounGender};\n",
        "pub const COMMON_NOUNS: [StaticCommonNoun; ",
        &common_nouns.len().to_string(),
        "] = [\n",
        &common_nouns.join(",\n"),
        "\n];",
    ]
    .join("");
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn select_common_nouns_works() {
        select_common_nouns();
    }
}
