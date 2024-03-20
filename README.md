# Panic4Rust

Panic4Rust is a collection of reproducible panics in Rust with the goal of advancing software engineering research, especially in Rust.

## Contents of Panic4Rust

### The crates

Panic4Rust contains 92 panics from Top-100 crates by all-time downloads in [crates.io](https://crates.io/) and manually constructed panics that cover some basic panic scenarios.

| Directory            | Crate                | Number of panics |
| -------------------- | -------------------- | ---------------- |
| lib-syn              | syn                  | 4                |
| lib-rand             | rand                 | 1                |
| lib-regex            | regex                | 6                |
| lib-aho-corasick     | aho-corasick         | 1                |
| lib-num-traits       | num-traits           | 1                |
| lib-clap             | clap                 | 4                |
| lib-serde_json       | serde_json           | 2                |
| lib-strsim           | strsim               | 1                |
| lib-time             | time                 | 2                |
| lib-percent-encoding | percent-encoding     | 2                |
| lib-textwrap         | textwrap             | 1                |
| lib-idna             | idna                 | 7                |
| lib-chrono           | chrono               | 13               |
| lib-uuid             | uuid                 | 1                |
| lib-nom              | nom                  | 5                |
| lib-hyper            | hyper                | 1                |
| lib-manual           | manually constructed | 40               |

### The panics

Each panic has the following properties:

- Fixed in a single commit.
- Fixed by modifying the source code (as opposed to configuration files, documentation, or test files).
- A triggering test exists in `main.rs` that failed before the fix and passes after the fix -- the test failure is not random or dependent on test execution order.
- Panic information(stack backtrace and panic info) and modified locations are provided for each panic.

## License

Apache-2.0 license, see [LICENSE](https://github.com/open-rust-initiative/Panic4Rust/blob/main/LICENSE) for more information.
