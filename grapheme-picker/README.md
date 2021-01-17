# Grapheme Picker

A Rust library to help users to get parts of words.

```toml
[dependencies]
grapheme-picker = "0.1.0"
```

```rust
use grapheme_picker::{take_last, take_lasts, drop_last, drop_lasts};
assert_eq!(take_last("tests"), "s");
assert_eq!(take_lasts("tests", 3), "sts");
assert_eq!(drop_last("tests"), "test");
assert_eq!(drop_lasts("tests", 3), "te");
```