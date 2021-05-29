///! Universally unique identifiers.
///!
///! The identifiers generated by this module:
///!
///! - are URL safe
///! - contain information on the type of object that they
///!   are identifying
///! - are roughly sortable by time of generation
///! - have an extremely low probability of collision
///!
///! Generated identifiers have a fixed length of 32 characters made up
///! of three parts separated by dots:
///!
///! - 2 characters in the range `[a-z]` that identifying the "family" of
///!   identifiers, usually the type of object the identifier is for
///!   e.g. `fi` = file, `re` = request
///!
///! - 10 characters in the range `[0-9a-f]` that are the hexadecimal encoding of the
///!   32bit number of seconds since Unix Timestamp Epoch 1500000000000 left padded
///!   with zeros
///!
///! - 18 characters in the range `[0-9A-Za-z]` that are randomly generated
///!
///! See
///!  - https://segment.com/blog/a-brief-history-of-the-uuid/
///!  - https://zelark.github.io/nano-id-cc/
///!  - https://gist.github.com/fnky/76f533366f75cf75802c8052b577e2a5
use nanoid::nanoid;
use std::convert::TryFrom;
use strum::ToString;

/// The available families of identifiers
#[derive(ToString)]
pub enum Family {
    #[strum(serialize = "fi")]
    File,
}

/// The epoch used for calculating the time stamp in the second part of
/// an identifier. Chosen as a recent time that was easily remembered
/// (happens to correspond to 2017-07-14T02:40:00.000Z).
const EPOCH: i64 = 1500000000;

/// The characters used in the third part of the identifier
const CHARACTERS: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z',
];

// Generate a universally unique identifier
pub fn generate(family: Family) -> String {
    let diff = chrono::Utc::now().timestamp() - EPOCH;
    let seconds =
        u32::try_from(diff).expect("Unable to convert to u32 (must be waaaay in the future");
    let rand = nanoid!(18, &CHARACTERS);
    format!("{}.{:010x}.{}", family.to_string(), seconds, rand)
}

#[cfg(test)]
mod tests {
    use eyre::Result;
    use regex::Regex;

    use super::*;

    #[test]
    fn test_generate() -> Result<()> {
        let id = generate(Family::File);

        assert_eq!(id.len(), 32);

        let re = Regex::new(r"[a-z]{2}\.[0-9a-f]{10}\.[0-9A-Za-z]{18}")?;
        assert!(re.is_match(&id));

        Ok(())
    }
}