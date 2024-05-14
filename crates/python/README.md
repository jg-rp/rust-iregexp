# I-Regexp checker

Check regular expressions for compliance with [RFC 9485](https://datatracker.ietf.org/doc/html/rfc9485).

## Install

```
python -m pip install i_regexp
```

## Usage

```python
from iregexp_check import check

check(r"[ab]{3}")  # True
check(r"[0-9]*?")  # False
```
