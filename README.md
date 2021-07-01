# hrclgbtqserver2020

This is a GraphQL server driven by data from the [Human Rights Campaign - State Equality Index 2020 Report](https://www.hrc.org/resources/state-equality-index).

> The State Equality Index (SEI) is a comprehensive state-by-state report that provides a review of statewide laws and policies that affect LGBTQ people and their families.

Data is presented in the API by state and by issue.

## Building

This is a Rust project that's built using Cargo.

```
cargo build
```

The build process will process the CSV file and generate code in `src/generated.rs`.

## Running

```
cargo run
```

## License

Dual licensed, MIT & Apache-2.0