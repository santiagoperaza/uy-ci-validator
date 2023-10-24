mod ci;

use crate::ci::CIError;
use clap::Parser;

#[derive(Parser, Debug)]
struct CliArgs {
    ci: u32,
    option: char,
}

fn main() {
  let CliArgs { ci, option } = CliArgs::parse();
  if option == 'v' {
      let is_valid = ci::validate(ci).map_err(|_e| CIError::ValidateInvalidFormat(ci.to_string()));
      match is_valid {
          Ok(true) => println!("✅ The CI {} is valid", ci),
          Ok(false) => println!("❌ The CI {} is not valid", ci),
          Err(e) => println!("{}", e)
      }
  } else if option == 'g' {
      let last_digit = ci::generate_last_digit(ci).map_err(|_e| CIError::GenerateLastDigitInvalidFormat(ci.to_string()));
      
      match last_digit {
          Ok(value) => println!("The generated last digit is: {}, the complete CI is: {}{}", value, ci, value),
          Err(e) => println!("{}", e)
      }
  } else {
      panic!("Invalid option {:?}", option)
  }
}