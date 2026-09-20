# Contributing

Contributions are welcome, and they are greatly appreciated! Every little bit helps, and credit will always be given.

## Get Started!

Ready to contribute? Here is how to set up **crc32-v2** for local development.

### Prerequisites

You will need the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain via `rustup`)
- [maturin](https://github.com/PyO3/maturin): `pip install maturin`
- Python 3.12 or above
- `git`

### Setup

1. Fork the `crc32-v2` repository on GitHub.

1. Clone your fork locally:

   ```sh
   git clone git@github.com:your_name_here/crc32-v2.git
   cd crc32-v2
   ```

1. Create a virtual environment and install the package in editable (development) mode:

   ```sh
   python3 -m venv .venv
   source .venv/bin/activate          # On Windows: .venv\Scripts\activate
   pip install maturin pytest
   maturin develop --features python
   ```

1. Create a branch for your bugfix or feature:

   ```sh
   git checkout -b name-of-your-bugfix-or-feature
   ```

### Running Tests

After making changes, run the full test suite:

```sh
# Rust unit tests + doc-tests
cargo test

# Python test suite
pytest tests/ -v

# no_std build (must compile cleanly)
cargo build --no-default-features
```

### Running Linters

Keep the code clean before submitting:

```sh
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

## Tips

Run a subset of the Python tests:

```sh
pytest tests/test_crc32.py -v
pytest tests/test_integration.py -v
pytest tests/ -k "digest" -v
```

Run a single Rust benchmark group:

```sh
cargo bench --bench benchmark -- crc32_little_16
cargo bench --bench nano_benchmark
```

Open the Rust API documentation locally:

```sh
cargo doc --open
```
