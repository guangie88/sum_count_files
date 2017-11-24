# `sum_count_files`

For summing all the integer value found in glob matching files.

Some test data are included at `data/`, and they are configured to be used for
unit/functional tests.

## Build Instructions

You will need to install both `rustc` + `cargo`. Follow the instructions at
[https://www.rustup.rs/](`https://www.rustup.rs/`) to set up `Rust` build
environment via `rustup` command. `nightly` environment is preferred if the
unit/functional tests are to be executed, otherwise `stable` should work for
only building the executable.

If you prefer to use a `Docker` build (for Linux environment only) instead, you
will not require the above but instead require `docker` and `docker-compose` to
be available. The steps to build using `Docker` are listed in
[Docker Build](#docker-build).

### Host Build

After the installation has been completed, simply run `cargo build --release` to
compile and get `sum_count_files` in `target/release/sum_count_files`.

If a fully statically build is preferred (on Linux only), you will need to
install additional target `x86_64-unknown-linux-musl`, via the `rustup` command
`rustup target add x86_64-unknown-linux-musl`, afterwhich, run the command
`cargo build --release --target x86_64-unknown-linux-musl`, and the executable
will be compiled and made available at
`target/x86_64-unknown-linux-musl/release/sum_count_files`.

### Docker Build

This uses a Docker container to perform the build + test, and allows the copying
back of release version `sum_count_files` executable. You will need `docker` and
`docker-compose` executables to perform the following commands below.

Run `docker compose -f docker-compose.yml build` to get both the compilation
image + runtime image.

* Run `docker compose -f docker-compose.yml run test` to further build and
  perform unit/function tests with `clippy` linting.
* Run `docker compose -f docker-compose.yml run copy` to copy the
  `sum_count_files` back into
  `target/x86_64-unknown-linux-musl/release/sum_count_files`. This executable is
  fully statically linked and should be runnable on most Linux distributions.

## Run Instructions

To run under `cargo`, run the command `cargo run --release -- -l
config/log_config.yml -g "data/st20170901/*.count"`. For more information, run
`cargo run --release -- --help`. You should be able to see the final summed
count value as a stdout output without any newline character.

To run directly, run the command `./target/release/sum_count_files -l
config/log_config.yml -g "data/st20170901/*.count"`

The `-l` flag is actually optional but useful for dynamically configurable
logging.

## Test Instructions

Run `cargo test --features clippy` to perform the tests in debug mode. This also
activates `clippy` which is a `Rust` linter and forces compilation error for
testing if there are any linting warnings.
