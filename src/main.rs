mod convert;

use crate::convert::to_arabic::{convert_to_arabic, validate_roman_input};
use crate::convert::to_roman::{convert_to_roman, validate_arabic_input};
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
        #[arg(value_parser = validate_roman_input)]
        roman: String,
    },
    /// Converts an Arabic number to Roman Numerals
    #[command(arg_required_else_help = true)]
    ToRoman {
        #[arg(value_parser = validate_arabic_input)]
        arabic: u16,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ToArabic { roman } => match convert_to_arabic(roman.to_string()) {
            Ok(val) => println!("{val}"),
            Err(e) => eprintln!("{e}"),
        },
        Commands::ToRoman { arabic } => match convert_to_roman(*arabic) {
            Ok(val) => println!("{val}"),
            Err(e) => eprintln!("{e}"),
        },
    }
}
