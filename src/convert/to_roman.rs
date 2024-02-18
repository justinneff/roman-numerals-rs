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

const FACTORS: &'static [(u16, [&'static str; 10])] = &[
    (1000, ["", "M", "MM", "MMM", "", "", "", "", "", ""]),
    (
        100,
        ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
    ),
    (
        10,
        ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
    ),
    (
        1,
        ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
    ),
];

pub fn convert_to_roman(input: u16) -> Result<String, String> {
    if ROMAN_RANGE.contains(&usize::from(input)) {
        let roman = FACTORS
            .into_iter()
            .fold(("".to_string(), input), |acc, fact| {
                let (base, numerals) = fact;
                let num = acc.1 / base;
                let result = acc.0 + numerals[usize::from(num)];
                let remainder = acc.1 - (base * num);
                (result.to_string(), remainder)
            })
            .0;
        Ok(roman.to_string())
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

    #[test]
    fn test_converting_to_roman() {
        assert_eq!(Ok("MCMXII".to_string()), convert_to_roman(1912));
        assert_eq!(Ok("MMXXI".to_string()), convert_to_roman(2021));
        assert_eq!(Ok("DCCLXXXIX".to_string()), convert_to_roman(789));
        assert_eq!(Ok("MDCCLXXVI".to_string()), convert_to_roman(1776));
        assert_eq!(Ok("MCMXVIII".to_string()), convert_to_roman(1918));
        assert_eq!(Ok("MCMXLIV".to_string()), convert_to_roman(1944))
    }

    #[test]
    fn test_converting_with_invalid_input() {
        assert_eq!(
            Err(String::from(
                "Arabic number is not in range for a Roman numeral 1-3999",
            )),
            convert_to_roman(0),
        );
        assert_eq!(
            Err(String::from(
                "Arabic number is not in range for a Roman numeral 1-3999",
            )),
            convert_to_roman(4000),
        );
    }
}
