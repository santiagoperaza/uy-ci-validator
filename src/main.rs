use clap::Parser;

#[derive(Parser, Debug)]
struct CliArgs {
    ci: u32,
    option: char,
}

fn main() {
    let CliArgs { ci, option } = CliArgs::parse();

    if option == 'v' {
        validate(ci)
    } else if option == 'g' {
        generate(ci)
    } else {
        panic!("Invalid option {:?}", option)
    }
}

fn validate(ci: u32) {
    println!("Validating CI {}", ci);
    let digits = ci.to_string().len();
    if digits != 8 {
        panic!("Invalid CI format");
    } else {
        let expected_last_digit = calculate_last_digit(ci);
        let last_digit = ci.to_string().chars().last().unwrap().to_digit(10).unwrap();
        if last_digit == expected_last_digit {
            println!("✅ The CI {} is valid", ci)
        } else {
            println!("❌ The CI {} is not valid", ci)
        }
    }
}

fn generate(ci: u32) {
    println!("Generating last digit for CI: {}", ci);
    let last_digit = calculate_last_digit(ci);
    println!("The generated last digit is: {}, the complete CI is: {}{}", last_digit, ci, last_digit);
}

fn calculate_last_digit(ci: u32) -> u32 {
    let digits: Vec<_> = ci.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let digits = &digits[..7];

    let validators = vec![8, 1, 2, 3, 4, 7, 6];
    let mut sum = 0;
    println!("Validating ci {}", ci);
    for (index, ele) in digits.iter().enumerate() {
        sum += validators[index] * ele
    }
    sum % 10
}