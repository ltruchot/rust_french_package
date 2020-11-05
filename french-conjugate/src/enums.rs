// You need to bring the trait into scope to use it!
use strum_macros::EnumIter;

#[derive(EnumIter, PartialEq)]
pub enum Person {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
}

#[derive(EnumIter, PartialEq)]
pub enum Tense {
    PresentIndicative,
}
