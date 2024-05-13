# Rust I-Regexp Checker (WIP)

Check regular expressions for compliance with [RFC 9485](https://datatracker.ietf.org/doc/html/rfc9485).

```rust
use iregexp::check;

fn main() {
    println!("{:?}", check(r"[0-9]*?"));  // false
}
```

## Contributing

TODO:

### Python bindings

As we're using a Rust [workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html), with multiple crates sharing a `target` directory and a root `Cargo.toml`, [`maturin`](https://www.maturin.rs/) commands need to specify the `-m` option, pointing to the `python` crate's `Cargo.toml` file.

For example, from the repository root, install the Python package locally during development with:

```
maturin develop -m crates/python/Cargo.toml
```
