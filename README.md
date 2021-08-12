# base79

This library provides a textual representation of base-79 fractional numbers with arbitrary precision, without a leading
zero or decimal point. Given any two base-79 numbers, this library can also give you the midpoint. This can be useful in
real-time collaborative applications and in applications where users can manually order records arbitrarily.

This is a fork of github.com/seungha-kim/base79 but simplified to 79 digits to not deal with some of the awkward
characters at the ends of the printable ASCII range.

[Documentation](https://docs.rs/base79)

## Example

```rust
use base79::Base79;
use std::str::FromStr;

fn main() {
    use base79::Base79;
    use std::str::FromStr;

    let n1 = Base79::mid();
    assert_eq!(n1.to_string(), "R");
    assert_eq!(n1.raw_digits(), vec![39]);

    let n2 = Base79::avg_with_zero(&n1);
    assert_eq!(n2.to_string(), ">");
    assert_eq!(n2.raw_digits(), vec![19]);

    let n3 = Base79::avg_with_one(&n1);
    assert_eq!(n3.to_string(), "f");

    let n4 = Base79::avg(&n1, &n2);
    assert_eq!(n4.to_string(), "H");

    let n5 = Base79::from_str("s?Q^Z").unwrap();
    assert_eq!(n5.raw_digits(), vec![72, 20, 38, 51, 47]);
}
```
