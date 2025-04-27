# DumbEq

dumb implementation of [`std::cmp::PartialEq`] and [`std::cmp::Eq`]

DumbEq is always false.

Example

```rust
use dumbeq::*;

#[derive(DumbEq, Debug)]
pub struct Same;

assert!(Same != Same, "not the same");
```
