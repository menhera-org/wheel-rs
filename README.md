# Wheel algebra library for Rust

```rust
use wheel::*;

let inf = w64::ONE / w64::ZERO;
assert_eq(w64::INFINITY, inf);
```

## Types
### Floating point wheel numbers
- `w32` (`Wheel32`)
- `w64` (`Wheel64`)

### Quotient wheel numbers
- `qw8` (`FractionWheel8')
- `qw16` (`FractionWheel16`)
- `qw32` (`FractionWheel32`)
- `qw64` (`FractionWheel64`)
- `qw128` (`FractionWheel128`)

## License

Apache 2.0 or MPL 2.0.

