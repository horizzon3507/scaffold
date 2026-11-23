//! Project name validation and display helpers.

use crate::error::Invalid;

/// Names use ASCII letters, digits, `-` and `_`, and start with a letter.
pub fn validate(name: &str) -> Result<(), Invalid> {
    if name.is_empty() {
        return Err(Invalid::new("project name", "it is empty"));
    }
    if !name.chars().next().unwrap().is_ascii_alphabetic() {
        return Err(Invalid::new("project name", "it must start with a letter"));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(Invalid::new(
            "project name",
            "use only ASCII letters, digits, '-' and '_'",
        ));
    }
    Ok(())
}

/// Turn `my-app` into `My App` for README titles.
pub fn title_case(name: &str) -> String {
    name.split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_kebab_and_snake_names() {
        assert!(validate("my-app").is_ok());
        assert!(validate("my_app2").is_ok());
    }

    #[test]
    fn rejects_empty_leading_digit_and_spaces() {
        assert!(validate("").is_err());
        assert!(validate("2cool").is_err());
        assert!(validate("no spaces").is_err());
    }

    #[test]
    fn title_cases_for_readme() {
        assert_eq!(title_case("my-app"), "My App");
        assert_eq!(title_case("scaffold"), "Scaffold");
    }
}
