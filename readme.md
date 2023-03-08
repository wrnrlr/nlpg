# Natural Language Processing for Postgresql

## Installation

Make sure to install both [`pgx`](https://crates.io/crates/pgx) and [`rust-bert`](https://crates.io/crates/rust-bert) correctly
and config your environment variables to be able to find the `libtorch` shared library.

Run example:

    LIBTORCH=/opt/homebrew/Cellar/pytorch/1.13.1;LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH;cargo pgx test

## Config
* `RUSTBERT_CACHE` location of language models defaults to `~/.cache/.rustbert`
* `PGX_IGNORE_RUST_VERSIONS`
