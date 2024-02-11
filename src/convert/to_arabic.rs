use phf::phf_map;
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

static ROMAN_VALUES: phf::Map<char, u16> = phf_map! {
    'I' => 1,
    'V' => 5,
    'X' => 10,
    'L' => 50,
    'C' => 100,
    'D' => 500,
    'M' => 1000,
};

pub fn convert_to_arabic(input: String) -> Result<u16, String> {
    let arabic_values: Result<Vec<u16>, String> = input
        .chars()
        .map(|c| match ROMAN_VALUES.get(&c).cloned() {
            Some(x) => Ok(x as u16),
            None => return Err(format!("Unknown roman character `{c}`")),
        })
        .collect();

    match arabic_values {
        Ok(values) => Ok(values
            .into_iter()
            .rfold((0, 0), |acc, val| {
                if val < acc.1 {
                    (acc.0 - val, val)
                } else {
                    (acc.0 + val, val)
                }
            })
            .0),
        Err(e) => Err(e),
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

    #[test]
    fn test_converting_to_arabic() {
        assert_eq!(Ok(1), convert_to_arabic("I".to_string()));
        assert_eq!(Ok(5), convert_to_arabic("V".to_string()));
        assert_eq!(Ok(10), convert_to_arabic("X".to_string()));
        assert_eq!(Ok(50), convert_to_arabic("L".to_string()));
        assert_eq!(Ok(100), convert_to_arabic("C".to_string()));
        assert_eq!(Ok(500), convert_to_arabic("D".to_string()));
        assert_eq!(Ok(1000), convert_to_arabic("M".to_string()));
        assert_eq!(Ok(9), convert_to_arabic("IX".to_string()));
        assert_eq!(Ok(14), convert_to_arabic("XIV".to_string()));
        assert_eq!(Ok(1776), convert_to_arabic("MDCCLXXVI".to_string()));
        assert_eq!(Ok(1918), convert_to_arabic("MCMXVIII".to_string()));
    }

    #[test]
    fn test_converting_with_invalid_characters() {
        assert_eq!(
            Err("Unknown roman character `N`".to_string()),
            convert_to_arabic("INI".to_string())
        )
    }
}
