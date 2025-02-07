# Non-Empty Containers

A simple set of non-empty alternatives to standard containers in Rust, including `NonEmptyVec`.

## Getting Started

Add this to your `Cargo.toml`:

```toml
# Cargo.toml

[dependencies]
nonempty-containers = "0.0.2"
```

The non-empty containers behave like their standard counterparts:

```rust
use nonempty-containers::NonEmptyVec;

let nev = NonEmptyVec::new(42, vec![1]);

nev.push(2);
nev.pop();
nev.pop();
assert_eq!(nev, NonEmptyVec::singleton(42));

// Errors!
nev.pop();
```

## Automatically Deriving `Arbitrary`

All non-empty containers can automatically derive `Arbitrary`, so long as the contained type 
also implements `Arbitrary`. This is useful for property-based testing, such as with `arbtest`.

```toml
# Cargo.toml

[dependencies]
nonempty-containers = { version = "0.0.2", features = ["arbitrary"] }
```

And then you can simply add `#[derive(Arbitrary)]` annotations to your types:

```rust
// pixels.rs

use arbitrary::Arbitrary;

#[derive(Arbitrary)]
pub struct Items(NonEmptyVec<u32>);
```

