use regex::Regex;
use std::ops::RangeInclusive;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "roman-numerals")]
#[command(about = "Tools for converting between Arabic numbers and Roman numerals.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Converts a Roman Numerals string to Arabic
    #[command(arg_required_else_help = true)]
    ToArabic {
        #[arg(value_parser = verify_roman_string)]
        roman: String,
    },
    /// Converts an Arabic number to Roman Numerals
    #[command(arg_required_else_help = true)]
    ToRoman {
        #[arg(value_parser = arabic_in_range)]
        arabic: u16,
    },
}

fn verify_roman_string(s: &str) -> Result<String, String> {
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

const ROMAN_RANGE: RangeInclusive<usize> = 1..=3999;

fn arabic_in_range(s: &str) -> Result<u16, String> {
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ToArabic { roman } => {
            println!("'roman-numerals to-arabic' was used, roman is: {roman:?}")
        }
        Commands::ToRoman { arabic } => {
            println!("'roman-numerals to-roman' was used, arabic is: {arabic:?}")
        }
    }
}
