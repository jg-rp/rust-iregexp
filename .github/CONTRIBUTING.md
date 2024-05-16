# Contributing to Rust I-Regexp

Hi. Your contributions and questions are always welcome. Feel free to ask questions, report bugs or request features on the [issue tracker](https://github.com/jg-rp/rust-iregexp/issues) or on [Github Discussions](https://github.com/jg-rp/rust-iregexp/discussions). Pull requests are welcome too.

**Table of contents**

- [Development](#development)
- [Documentation](#documentation)
- [Style Guides](#style-guides)

## Development

We're using a Rust [workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html) with two crates.

- `crates/iregexp` is a [pest](https://github.com/pest-parser)-based I-Regexp parser.
- `crates/python` is Python bindings for `crates/iregexp` using [Maturin](https://github.com/PyO3/maturin) and [PyO3](https://github.com/PyO3/pyo3).

### Rust

`crates/iregexp` is the one and only default member in the workspace.

Run tests with cargo.

```shell
$ cargo test
```

Lint with clippy.

```shell
$ cargo clippy
```

Build with cargo.

```shell
$ cargo build
```

### Python

As we're using a Rust [workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html), [`maturin`](https://www.maturin.rs/) commands need to specify the `-m` option, pointing to the `python` crate's `Cargo.toml` file.

For example, from the repository root, install the Python package locally during development with:

```
maturin develop -m crates/python/Cargo.toml
```

Were currently relying on [jg-rp/python-jsonpath-rfc9535](https://github.com/jg-rp/python-jsonpath-rfc9535/blob/main/tests/test_iregexp.py) to test the Python package.

## Documentation

Documentation is currently in the [README](https://github.com/jg-rp/rust-iregexp/blob/main/README.md) and project source code only.

## Style Guides

### Git Commit Messages

There are no hard rules for git commit messages, although you might like to indicate the type of commit by starting the message with `docs:`, `chore:`, `feat:`, `fix:` or `refactor:`, for example.

### Python Style

All Python files are formatted using [Black](https://github.com/psf/black), with its default configuration.

Docstrings must use [Google style docstrings](https://sphinxcontrib-napoleon.readthedocs.io/en/latest/example_google.html).
