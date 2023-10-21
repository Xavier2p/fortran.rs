//! Validation module.
//!
//! This module contains the functions to validate the syntax of the code.
use crate::helpers::errors::Error;

#[allow(dead_code)]
/// This struct contains the format of the error and the message.
pub struct ValidationError {
    /// This field contains type of the error.
    error: Error,

    /// This field contains the message of the error.
    message: String,
}

/// This function validates the name of the identifier.
/// ## Conditions:
/// - The name must be between 1 and 31 characters.
/// - The name must start with a letter.
/// - The name must contain only alphanumeric and `_` characters.
///
/// ## Errors:
/// - `ValidationError::WrongFormat` if the name is not between 1 and 31 characters or if it does not start with a letter.
/// - `ValidationError::ForbiddenCharacter` if the name contains a forbidden character.
#[allow(dead_code)]
pub fn identifier_name(name: String) -> Result<(), ValidationError> {
    if name.len() > 31 || name.is_empty() {
        Err(ValidationError {
            error: Error::WrongFormat,
            message: format!(
                "Identifier name must be between 1 and 31 characters, got {}.",
                name.len()
            ),
        })
    } else if name.starts_with(|c: char| !c.is_alphabetic() || c == '_') {
        Err(ValidationError {
            error: Error::WrongFormat,
            message: format!(
                "Identifier name must start with a letter, got {}.",
                name.chars().next().unwrap()
            ),
        })
    } else if !name
        .chars()
        .all(|c: char| c.is_ascii() && c.is_alphanumeric() || c == '_')
    {
        Err(ValidationError {
            error: Error::ForbiddenCharacter,
            message: format!(
                "Identifier name must contain only alphanumeric and `_` characters, got {}.",
                name
            ),
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("a", true; "one letter")]
    #[test_case("a1", true; "one letter and one number")]
    #[test_case("ab", true; "two letters")]
    #[test_case("1a", false; "starts with a number")]
    #[test_case("a_", true; "ends with an underscore")]
    #[test_case("_a", false; "starts with an underscore")]
    #[test_case("aéb", false; "contains accent")]
    #[test_case("a#", false; "contains non alphanumeric character")]
    #[test_case("a1234567890qwertyuiopasdfghjklzxcvb", false; "more than 31 characters")]
    fn test_identifier_name(name: &str, expected: bool) {
        assert_eq!(identifier_name(name.to_string()).is_ok(), expected);
    }
}
