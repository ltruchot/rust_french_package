use french_words::structs::NounGender;

#[derive(Debug, Ord, PartialEq, Eq, PartialOrd)]
pub struct Lemma {
    pub content: String,
    pub phonetic: Option<String>,
}

pub struct CommonNoun {
    pub gender: Option<String>,
    pub singular: Lemma,
    pub plural: Option<Lemma>,
    pub feminine: Option<Lemma>,
    pub feminine_plural: Option<Lemma>,
}
impl CommonNoun {
    pub fn get_as_str(&self) -> String {
        let phonetic = match &self.singular.phonetic {
            Some(res) => ["Some(\"", res, "\")"].join(""),
            None => String::from("None"),
        };
        let gender = match self.gender.as_ref().unwrap_or(&String::from("")).as_str() {
            "masculine" => NounGender::Masculine,
            "feminine" => NounGender::Feminine,
            "invariable" => NounGender::Invariable,
            _ => NounGender::Unknown,
        };
        let plural = match &self.plural {
            Some(res) => [
                "Some(StaticLemma { content: \"",
                &res.content,
                "\", phonetic: ",
                &match &res.phonetic {
                    Some(ph) => ["Some(\"", ph, "\""].join(""),
                    None => String::from("None"),
                },
                ") })",
            ]
            .join(""),
            None => String::from("None"),
        };

        [
            "    StaticCommonNoun {",
            "\n        gender: ",
            "NounGender::",
            &gender.to_string(),
            ",",
            "\n        singular: StaticLemma { content: \"",
            &self.singular.content,
            "\", phonetic: ",
            &phonetic,
            " },",
            "\n        plural: ",
            &plural,
            "\n    }",
        ]
        .join("")
    }
}
