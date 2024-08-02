# Panic4R

Panic4R is a collection of reproducible panics in Rust with the goal of advancing software engineering research, especially in Rust.

## Contents of Panic4R

### The crates

Panic4R contains 102 panics from Top-500 crates by all-time downloads in [crates.io](https://crates.io/).

| Directory               | Crate               | Number of panics |
| ----------------------- | ------------------- | ---------------- |
| lib-syn                 | syn                 | 4                |
| lib-rand                | rand                | 2                |
| lib-regex               | regex               | 6                |
| lib-aho-corasick        | aho-corasick        | 1                |
| lib-num-traits          | num-traits          | 1                |
| lib-clap                | clap                | 4                |
| lib-serde_json          | serde_json          | 2                |
| lib-strsim              | strsim              | 1                |
| lib-time                | time                | 2                |
| lib-idna                | idna                | 7                |
| lib-hashbrown           | hashbrown           | 1                |
| lib-proc-macro2         | proc-macro2         | 3                |
| lib-smallvec            | smallvec            | 2                |
| lib-percent-encoding    | percent-encoding    | 2                |
| lib-textwrap            | textwrap            | 1                |
| lib-chrono              | chrono              | 13               |
| lib-uuid                | uuid                | 1                |
| lib-nom                 | nom                 | 5                |
| lib-tokio               | tokio               | 5                |
| lib-hyper               | hyper               | 1                |
| lib-futures             | futures             | 5                |
| lib-toml                | toml                | 2                |
| lib-httparse            | httparse            | 1                |
| lib-object              | object              | 1                |
| lib-rustc-demangle      | rustc-demangle      | 1                |
| lib-rustls              | rustls              | 1                |
| lib-form_urlencoded     | form_urlencoded     | 1                |
| lib-gimli               | gimli               | 1                |
| lib-reqwest             | reqwest             | 1                |
| lib-num-bigint          | num-bigint          | 1                |
| lib-rayon               | rayon               | 1                |
| lib-bumpalo             | bumpalo             | 2                |
| lib-filetime            | filetime            | 1                |
| lib-fixedbitset         | fixedbitset         | 1                |
| lib-phf                 | phf                 | 1                |
| lib-prost               | prost               | 1                |
| lib-pest                | pest                | 1                |
| lib-serde-yaml          | serde-yaml          | 1                |
| lib-libm                | libm                | 1                |
| lib-prettyplease        | prettyplease        | 1                |
| lib-bytemuck            | bytemuck            | 1                |
| lib-cargo_metadata      | cargo_metadata      | 1                |
| lib-tinytemplate        | tinytemplate        | 1                |
| lib-tar                 | tar                 | 1                |
| lib-plotters            | plotters            | 2                |
| lib-pretty_assertion    | pretty_assertion    | 1                |
| lib-yansi               | yansi               | 1                |
| lib-crossbeam           | crossbeam           | 1                |
| lib-brotli-decompressor | brotli-decompressor | 1                |
| lib-indicatif           | indicatif           | 1                |
| lib-md5                 | md5                 | 1                |

### The panics

Each panic has the following properties:

- Fixed in a single commit.
- Fixed by modifying the source code (as opposed to configuration files, documentation, or test files).
- A triggering test exists in `main.rs` that failed before the fix and passes after the fix -- the test failure is not random or dependent on test execution order.
- Panic information(stack backtrace and panic info) and modified locations are provided for each panic.
- Official fixing location in `perfect_location`.

## License

Apache-2.0 license, see [LICENSE](https://github.com/open-rust-initiative/Panic4Rust/blob/main/LICENSE) for more information.
