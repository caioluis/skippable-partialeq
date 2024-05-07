# timeless-partialeq
## PartialEq, but ignores timestamps, ideal for API testing

This crate provides a custom derive macro `TimelessPartialEq` that allows you to implement `PartialEq` for structs while ignoring fields ending with `_at`.

## Usage
First, add TimelessPartialEq as a dependency.

```zsh
cargo add timeless-partialeq
```

Then, derive `TimelessPartialEq` for your struct:

```rust
use timeless_partialeq::TimelessPartialEq;

#[derive(Debug, TimelessPartialEq)]
pub struct Post {
    pub id: i64,
    pub content: String,
    pub author: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
```

This will generate an implementation of `PartialEq` that ignores the fields ending with `_at`, while still checking for `Option`'s outer `None` & `Some` values.

## Limitations

Currently, it only assert fields (or should I say it does not assert) that end with `_at`, thus requiring that your structs are forced to be defined that way. It would be a great addition to the current tests if we could check that struct fields that are renamed with another derive proc macro (e.g. a struct that represents a SQLx table fields) also work with TimelessPartialEq.
