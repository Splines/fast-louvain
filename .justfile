# List all available commands.
help:
    @just --list


################################# Build & Run ##################################

# Read version from Cargo.toml
# taken from here: https://github.com/flosse/clean-architecture-with-rust/blob/master/justfile#L39
version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/v\1/p' Cargo.toml | head -1`
# Print the current version of the project
version:
    @echo {{version}}

alias rd := run-dev
# Run the project in development mode.
run-dev *ARGS:
    cargo run --profile dev {{ARGS}}

alias rr := run-release
# Run the project in release mode.
run-release *ARGS:
    cargo run --release {{ARGS}}

alias b := build
# Build the project in release mode (not necessary for running).
build:
    cargo build --release


################################# Development ##################################

# Run all unit tests of every workspace
alias t := test
test:
    cargo test --lib --locked --workspace

alias c := clippy
# Get code suggestions from clippy (collection of Rust lints).
clippy:
    cargo clippy --all -- -D warnings


############################### Comparison #####################################
# Compare to [original C++ Louvain implementation](https://sites.google.com/site/findcommunities/home).
# This has to be # executed in a Linux environment where `tar`, `make` etc. are available.
# Try WSL if you're on Windows.
# Compare the results of the original C++ implementation to our implementation.
compare:
    python3 ./tests/compare_to_original_implementation.py 


################################# Documentation ################################

# We use the [`mdbook-katex`](https://github.com/lzanini/mdbook-katex)
# preprocessor, so you should install it first.
# Open docs in browser.
docs:
    @echo "⚠ Make sure you have installed mdbook-katex via 'cargo install mdbook-katex'"
    mdbook watch ./docs --open
