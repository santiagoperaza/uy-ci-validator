use std::fmt;

use clap::Parser;

#[derive(Parser, Debug)]
struct CliArgs {
    ci: u32,
    option: char,
}

#[derive(Debug)]
pub enum CIError {
    /// Represents an error while validating a CI
    ValidateInvalidFormat(String),
    /// Represents an error while generating the last digit of a CI
    GenerateLastDigitInvalidFormat(String),
}

impl fmt::Display for CIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            CIError::ValidateInvalidFormat(ci) => format!("Invalid validation format, the CI must be 8 digits: {}", ci),
            CIError::GenerateLastDigitInvalidFormat(ci) => format!("Invalid last digit generation format, the CI must be 7 digits: {}", ci)
        };
        write!(f, "{}", message)
    }
}

fn main() {
    let CliArgs { ci, option } = CliArgs::parse();
    if option == 'v' {
        let is_valid = validate(ci).map_err(|_e| CIError::ValidateInvalidFormat(ci.to_string()));
        match is_valid {
            Ok(true) => println!("✅ The CI {} is valid", ci),
            Ok(false) => println!("❌ The CI {} is not valid", ci),
            Err(e) => println!("{}", e)
        }
    } else if option == 'g' {
        let last_digit = generate_last_digit(ci).map_err(|_e| CIError::GenerateLastDigitInvalidFormat(ci.to_string()));
        
        match last_digit {
            Ok(value) => println!("The generated last digit is: {}, the complete CI is: {}{}", value, ci, value),
            Err(e) => println!("{}", e)
        }
    } else {
        panic!("Invalid option {:?}", option)
    }
}

fn validate(ci: u32) -> Result<bool, CIError> {
    let digits = ci.to_string().len();
    if digits != 8 {
        Err(CIError::ValidateInvalidFormat(ci.to_string()))
    } else {
        println!("Validating CI {}", ci);
        let expected_last_digit = calculate_last_digit(ci);
        let last_digit = ci.to_string().chars().last().unwrap().to_digit(10).unwrap();
        if last_digit == expected_last_digit {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn generate_last_digit(ci: u32) -> Result<u32, CIError> {
    let digits = ci.to_string().len();
    if digits != 7 {
        Err(CIError::GenerateLastDigitInvalidFormat(ci.to_string()))
    } else {
        println!("Generating last digit for CI: {}", ci);
        let last_digit = calculate_last_digit(ci);
        Ok(last_digit)
    }
}

fn calculate_last_digit(ci: u32) -> u32 {
    let digits: Vec<_> = ci.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let digits = &digits[..7];

    let validators = vec![8, 1, 2, 3, 4, 7, 6];
    let mut sum = 0;
    for (index, ele) in digits.iter().enumerate() {
        sum += validators[index] * ele
    }
    sum % 10
}