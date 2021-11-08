///! Utilities for universally unique identifiers.
///!
///! The identifiers generated by this module:
///!
///! - are URL safe
///! - contain information on the type of object that they
///!   are identifying
///! - have an extremely low probability of collision
///!
///! Generated identifiers have a fixed length of 32 characters made up
///! of two parts separated by a hyphen:
///!
///! - 2 characters in the range `[a-z]` that identifying the "family" of
///!   identifiers, usually the type of object the identifier is for
///!   e.g. `fi` = file, `re` = request
///!
///! - 20 characters in the range `[0-9A-Za-z]` that are randomly generated
///!
///! For project identifiers (those starting with 'pr') only lowercase
///! letters are used for compatibility with Docker image naming rules.
///!
///! The total size of the generated ids is 23 bytes which allows it to fit
///! inside a [`SmartString`](https://lib.rs/crates/smartstring) for better
///! performance that a plain old `String`.
///!
///! See
///!  - https://segment.com/blog/a-brief-history-of-the-uuid/
///!  - https://zelark.github.io/nano-id-cc/
///!  - https://gist.github.com/fnky/76f533366f75cf75802c8052b577e2a5
use eyre::{bail, Result};
use nanoid::nanoid;
use regex::Regex;
use smartstring::{Compact, SmartString};

pub type Uuid = SmartString<Compact>;

/// The separator between the family and random parts of the identifier
///
/// A hyphen provides for better readability than a dot or colon when used
/// in pubsub topic strings and elsewhere.
const SEPARATOR: &str = "-";

/// The characters used in the random part of the identifier
const CHARACTERS: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

/// Create a family of UUIDs
///
/// ```
/// use uuid_utils::uuid_family;
///
/// uuid_family!(MyId, "my");
/// let id = MyId::new();
/// ```
#[macro_export]
macro_rules! uuid_family {
    ($name:ident, $family:literal) => {
        #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
        struct $name(uuid_utils::Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(uuid_utils::generate($family))
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "{}", self.0.to_string())
            }
        }

        impl std::cmp::PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                self.0 == other.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = uuid_utils::Uuid;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

// Generate a universally unique identifier
pub fn generate(family: &str) -> Uuid {
    let chars = nanoid!(20, &CHARACTERS);
    [family, SEPARATOR, &chars].concat().into()
}

// Generate a universally unique identifier with only lowercase letters and digits
pub fn generate_lower(family: &str) -> Uuid {
    let chars = nanoid!(20, &CHARACTERS[..36]);
    [family, SEPARATOR, &chars].concat().into()
}

// Test whether a string is an identifer for a particular family
pub fn matches(family: &str, id: &str) -> bool {
    let re = [family, SEPARATOR, "[0-9a-zA-Z]{20}"].concat();
    let re = Regex::new(&re).expect("Should be a valid regex");
    re.is_match(id)
}

// Assert that a `Uuid` is an identifer for a particular family
pub fn assert(family: &str, id: Uuid) -> Result<Uuid> {
    match matches(family, &id) {
        true => Ok(id),
        false => bail!(
            "Invalid UUID `{}`, family does not match `{}`",
            family.to_string(),
            id
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        let id = generate("no");
        assert_eq!(id.len(), 23);
        assert!(matches("no", &id));
        assert("no", id).unwrap();
    }

    #[test]
    fn lower() {
        let id = generate_lower("pr");
        assert_eq!(id.len(), 23);
        assert!(matches("pr", &id));
        assert("pr", id).unwrap();
    }
}
