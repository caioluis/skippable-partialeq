# timeless-partialeq
## PartialEq, but ignores timestamps, ideal for API testing

This crate provides a custom derive macro `TimelessPartialEq` that allows you to implement `PartialEq` for structs while ignoring fields based on their suffixes.

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

You can also use the `#[exclude_suffix]` attribute to filter by specific suffixes:

```rust
use timeless_partialeq::TimelessPartialEq;

#[derive(Debug, TimelessPartialEq)]
#[exclude_suffix(at, date)]
pub struct Post {
    pub id: i64,
    pub content: String,
    pub author: i32,
    pub creation_date: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
```

This would exclude fields ending with `_at` and/or `date`.


# About the crate
This crate was made to solve a very specific problem: assert the equality of two objects despite the timestamp differences. It was also made so that I could study proc macros.
However, just after a day after publishing it, I realized that it can be broader than just timestamps.

I will not make a commitment into iterating this quickly, but it is in my plans to expand the scope of the crate.
