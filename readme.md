# Natural Language Processing for Postgresql

### Prerequazites

* Ubuntu: `sudo apt install build-essential libclang-dev libreadline-dev zlib1g-dev flex bison libxml2-dev libxslt-dev libssl-dev libxml2-utils xsltproc ccache postgresql-server-dev-15 -y`

1. Install [libtorch](https://pytorch.org/get-started/locally/) \
   https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-1.13.1%2Bcpu.zip
2. `cargo install --locked cargo-pgx`
3. `cargo pgx init`

This extension can be used together with the pgvector extension.
Pgvector must be compiled from source and to copied to the pgx test installation of postgresql.

    cp vector.so ~/.pgx/15.2/pgx-install/lib/postgresql/
    cp vector.control ~/.pgx/15.2/pgx-install/share/postgresql/extension/
    cp sql/vector*.sql ~/.pgx/15.2/pgx-install/share/postgresql/extension/

## Installation

Make sure to install both [`pgx`](https://crates.io/crates/pgx) and [`rust-bert`](https://crates.io/crates/rust-bert) correctly
and config your environment variables to be able to find the `libtorch` shared library.

Run example:

    LD_LIBRARY_PATH=/home/werner/Code/libtorch/lib:$LD_LIBRARY_PATH cargo pgx run

Install package

    LD_LIBRARY_PATH=/home/werner/Code/libtorch/lib:$LD_LIBRARY_PATH sudo cargo pgx install

## Config
* `RUSTBERT_CACHE` location of language models defaults to `~/.cache/.rustbert`
* `PGX_IGNORE_RUST_VERSIONS`

## Awesome links

* [Postgresql Internals](https://postgrespro.com/community/books/internals)
* [The Wonders of Postgres Logical Decoding Messages](https://www.infoq.com/articles/wonders-of-postgres-logical-decoding-messages/)
* [Neural Networks: Zero to Hero](https://karpathy.ai/zero-to-hero.html), ML course by Andrej Karpathy