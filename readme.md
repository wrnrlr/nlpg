# Natural Language Processing for Postgresql

## Development

This extension can be used together with the pgvector extension.
Pgvector must be compiled from source and to copied to the pgx test intallsion of postgresql. 

    cp vector.so ~/.pgx/15.2/pgx-install/lib/postgresql/
    cp vector.control ~/.pgx/15.2/pgx-install/share/postgresql/extension/
    cp sql/vector*.sql ~/.pgx/15.2/pgx-install/share/postgresql/extension/

## Installation

Make sure to install both [`pgx`](https://crates.io/crates/pgx) and [`rust-bert`](https://crates.io/crates/rust-bert) correctly
and config your environment variables to be able to find the `libtorch` shared library.

Run example:

    LIBTORCH=/opt/homebrew/Cellar/pytorch/1.13.1;LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH;cargo pgx test

## Config
* `RUSTBERT_CACHE` location of language models defaults to `~/.cache/.rustbert`
* `PGX_IGNORE_RUST_VERSIONS`

export LIBTORCH=/home/werner/Code/libtorch
export LD_LIBRARY_PATH=$LIBTORCH/lib:$LD_LIBRARY_PATH

LD_LIBRARY_PATH=/home/werner/Code/libtorch/lib:$LD_LIBRARY_PATH cargo pgx run


