#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use unicode_segmentation::UnicodeSegmentation;

pub fn take_last(word: &str) -> String {
    let s = String::from(word);
    // check last
    let last = s.graphemes(true).last();
    String::from(match last {
        Some(letter) => letter,
        None => "",
    })
}

pub fn take_lasts(word: &str, n: usize) -> String {
    // chars
    let chars = word.graphemes(true).collect::<Vec<&str>>();
    // length of chars
    let len = chars.len();
    // skip "n" and return this tail
    let to_skip = if len >= n { len - n } else { 0 };
    chars
        .iter()
        .map(AsRef::as_ref)
        .skip(to_skip)
        .collect::<Vec<&str>>()
        .join("")
}

pub fn drop_last(word: &str) -> String {
    drop_lasts(word, 1)
}

pub fn drop_lasts(word: &str, n: usize) -> String {
    // chars
    let chars = word.graphemes(true).collect::<Vec<&str>>();
    // length of chars
    let len = chars.len();

    // take "n" and return this head
    let to_skip = if len >= n { len - n } else { 0 };
    chars
        .iter()
        .map(AsRef::as_ref)
        .take(to_skip)
        .collect::<Vec<&str>>()
        .join("")
}

#[cfg(test)]
#[allow(clippy::non_ascii_literal)]
mod tests {
    use super::*;

    #[test]
    fn take_last_works() {
        assert_eq!(take_last("tests"), "s");
        assert_eq!(take_last("xârâcùù"), "ù");
        assert_eq!(take_last("a"), "a");
        assert_eq!(take_last(""), "");
    }

    #[test]
    fn take_lasts_works() {
        assert_eq!(take_lasts("tests", 0), "");
        assert_eq!(take_lasts("xârâcùù", 0), "");
        assert_eq!(take_lasts("a", 0), "");
        assert_eq!(take_lasts("", 0), "");

        assert_eq!(take_lasts("tests", 1), "s");
        assert_eq!(take_lasts("xârâcùù", 1), "ù");
        assert_eq!(take_lasts("a", 1), "a");
        assert_eq!(take_lasts("", 1), "");

        assert_eq!(take_lasts("tests", 2), "ts");
        assert_eq!(take_lasts("xârâcùù", 2), "ùù");
        assert_eq!(take_lasts("a", 2), "a");
        assert_eq!(take_lasts("", 2), "");

        assert_eq!(take_lasts("constitution", 10), "nstitution");
        assert_eq!(take_lasts("xârâcùù", 10), "xârâcùù");
        assert_eq!(take_lasts("a", 10), "a");
        assert_eq!(take_lasts("", 10), "");
    }

    #[test]
    fn drop_last_works() {
        assert_eq!(drop_last("tests"), "test");
        assert_eq!(drop_last("xârâcùù"), "xârâcù");
        assert_eq!(drop_last("a"), "");
        assert_eq!(drop_last(""), "");
    }

    #[test]
    fn drop_lasts_works() {
        assert_eq!(drop_lasts("tests", 0), "tests");
        assert_eq!(drop_lasts("xârâcùù", 0), "xârâcùù");
        assert_eq!(drop_lasts("a", 0), "a");
        assert_eq!(drop_lasts("", 0), "");

        assert_eq!(drop_lasts("tests", 1), "test");
        assert_eq!(drop_lasts("xârâcùù", 1), "xârâcù");
        assert_eq!(drop_lasts("a", 1), "");
        assert_eq!(drop_lasts("", 1), "");

        assert_eq!(drop_lasts("tests", 2), "tes");
        assert_eq!(drop_lasts("xârâcùù", 2), "xârâc");
        assert_eq!(drop_lasts("a", 2), "");
        assert_eq!(drop_lasts("", 2), "");

        assert_eq!(drop_lasts("constitution", 10), "co");
        assert_eq!(drop_lasts("xârâcùù", 10), "");
        assert_eq!(drop_lasts("a", 10), "");
        assert_eq!(drop_lasts("", 10), "");
    }
}
