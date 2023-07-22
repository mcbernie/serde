//! Ensures that error message points to the path in attribute
use serde_derive::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "tag")]
enum Enum {
    // Newtype variants does not use the provided path, so it is forbidden here
    // Newtype(#[serde(default = "main")] u8),
    // Tuple variants does not supported in internally tagged enums
    Struct {
        #[serde(default = "main")]
        f1: u8,
        f2: u8,
        #[serde(default = "main")]
        f3: i8,
    },
}

fn main() {}
