# Uruguayan ID Card Validator

This repository provides validation of Uruguayan ID Card, aka CI.

The format of the Uruguayan ID Card consists of 7 digits plus a verification digit that is the result of an arithmetic operation based on the initial digits.

## Instructions

This CLI tool allows to either validate or generate the last digit of a CI.

### Validate

To validate a CI it is required to provide the following arguments:
- CI with its 8 digits
- `v` which stands for validate

Exmple: `cargo run -- 45995892 v`

### Generate last digit

To generate the last digit of a CI it is required to provide the following arguments:
- CI with its 7 digits, as the last digit is going to be generated
- `g` which stands for generate

Exmple: `cargo run -- 4599589 g`

### Unit Tests

Execute `cargo test`