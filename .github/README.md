# Rust I-Regexp Checker

Check regular expressions for compliance with [RFC 9485](https://datatracker.ietf.org/doc/html/rfc9485).

**Table of Contents**

- [Rust install](#rust-install)
- [Rust usage](#rust-usage)
- [Python install](#python-install)
- [Python usage](#python-usage)
- [Contributing](#contributing)
- [License](#license)

## Rust install

```
cargo add iregexp
```

## Rust usage

```rust
use iregexp::check;

fn main() {
    println!("{:?}", check(r"[0-9]*?"));  // false
}
```

## Python install

```
pip install iregexp_check
```

## Python usage

```python
from iregexp_check import check

print(check(r"[ab]{3}"))  # True
print(check(r"[0-9]*?"))  # False
```

## Contributing

See [CONTRIBUTING.md](https://github.com/jg-rp/rust-iregexp/blob/main/.github/CONTRIBUTING.md)

## License

[MIT](https://spdx.org/licenses/MIT.html) license.
