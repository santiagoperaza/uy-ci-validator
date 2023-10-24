
use std::fmt;

const CI_MULTIPLIERS: [u32; 7] = [8, 1, 2, 3, 4, 7, 6];

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

pub fn validate(ci: u32) -> Result<bool, CIError> {
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

pub fn generate_last_digit(ci: u32) -> Result<u32, CIError> {
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

    let mut sum = 0;
    for (index, ele) in digits.iter().enumerate() {
        sum += CI_MULTIPLIERS[index] * ele
    }
    sum % 10
}

#[cfg(test)]
mod test;
