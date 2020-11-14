#[derive(Debug)]
pub enum NounGender {
    Masculine,
    Feminine,
    Invariable,
    Unknown,
}
impl std::fmt::Display for NounGender {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub struct StaticLemma {
    pub content: &'static str,
    pub phonetic: Option<&'static str>,
}
pub struct StaticCommonNoun {
    pub gender: NounGender,
    pub singular: StaticLemma,
    pub plural: Option<StaticLemma>,
}
