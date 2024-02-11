use std::ops::RangeInclusive;

const ROMAN_RANGE: RangeInclusive<usize> = 1..=3999;

pub fn validate_arabic_input(s: &str) -> Result<u16, String> {
    let arabic: usize = s.parse().map_err(|_| format!("`{s}` is not a number"))?;
    if ROMAN_RANGE.contains(&arabic) {
        Ok(arabic as u16)
    } else {
        Err(format!(
            "Arabic number is not in range for a Roman numeral {}-{}",
            ROMAN_RANGE.start(),
            ROMAN_RANGE.end(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_arabic_string() {
        assert_eq!(Ok(32), validate_arabic_input("32"));
    }

    #[test]
    fn test_validate_string_with_letters() {
        assert_eq!(
            Err(String::from("`3n` is not a number")),
            validate_arabic_input("3n")
        );
    }

    #[test]
    fn test_validate_in_range_limits() {
        assert_eq!(Ok(1), validate_arabic_input("1"));
        assert_eq!(Ok(3999), validate_arabic_input("3999"));
    }

    #[test]
    fn test_validate_out_of_range_values() {
        assert_eq!(
            Err(String::from(
                "Arabic number is not in range for a Roman numeral 1-3999"
            )),
            validate_arabic_input("0")
        );
        assert_eq!(
            Err(String::from(
                "Arabic number is not in range for a Roman numeral 1-3999"
            )),
            validate_arabic_input("4000")
        );
    }
}
