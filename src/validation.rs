use crate::helpers::errors::Error;

#[allow(dead_code)]
pub struct ValidationError {
    error: Error,
    message: String,
}

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
