#[cfg(test)]
use super::*;

#[test]
fn assert_validate_returns_true() {
    let valid_ci = 45995892;
    let result = validate(valid_ci).unwrap();
    assert_eq!(true, result);
}

#[test]
fn assert_validate_returns_false() {
    let invalid_ci = 45995899;
    let result = validate(invalid_ci).unwrap();
    assert_eq!(false, result);
}

#[test]
fn assert_validation_fails_due_to_invalid_ci_format() {
    let invalid_ci_format = 459958921;
    let last_digit = generate_last_digit(invalid_ci_format);
    assert!(last_digit
        .unwrap_err()
        .to_string()
        .starts_with("Invalid last digit generation format"));
}

#[test]
fn assert_generates_last_digit_successfully() {
    let valid_ci = 4599589;
    let last_digit = generate_last_digit(valid_ci).unwrap();
    assert_eq!(2, last_digit);
}

#[test]
fn assert_generates_last_digit_fails_due_to_invalid_ci_format() {
    let invalid_ci_format = 45995891;
    let last_digit = generate_last_digit(invalid_ci_format);
    assert!(last_digit
        .unwrap_err()
        .to_string()
        .starts_with("Invalid last digit generation format"));
}
