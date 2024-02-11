use regex::Regex;

pub fn validate_roman_input(s: &str) -> Result<String, String> {
    let roman = s.to_uppercase();
    let re = Regex::new("[^CDILMVX]").unwrap();
    if !re.is_match(&roman) {
        Ok(roman)
    } else {
        Err(format!(
            "Input `{s}` contains invalid Roman characters. Should only contain C, D, I, L, M, V, X.",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_roman_string() {
        assert_eq!(Ok(String::from("MCDLXIV")), validate_roman_input("MCDLXIV"));
    }

    #[test]
    fn test_validate_with_invalid_characters() {
        assert_eq!(
            Err(String::from(
                "Input `XIVT` contains invalid Roman characters. Should only contain C, D, I, L, M, V, X."
            )),
            validate_roman_input("XIVT")
        );
    }

    #[test]
    fn test_validate_returns_uppercase_input() {
        assert_eq!(Ok(String::from("XIV")), validate_roman_input("xiv"));
    }
}
