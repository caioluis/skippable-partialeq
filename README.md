# skippable-partialeq
## PartialEq, but you can ignore fields 

This crate provides a custom derive macro `SkippablePartialEq` that allows you to implement `PartialEq` for structs while ignoring fields based on custom rules such as suffixes, as well as specific fields.

## Usage
First, add SkippablePartialEq as a dependency.

```zsh
cargo add skippable-partialeq
```

Then, derive `SkippablePartialEq` for your struct:

```rust
use skippable_partialeq::SkippablePartialEq;

#[derive(Debug, SkippablePartialEq)]
#[exclude_suffix(at)]
pub struct Post {
    pub id: i64,
    pub content: String,
    pub author: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
```

This will generate an implementation of `PartialEq` that ignores the fields ending with `_at`, while still checking for `Option`'s outer `None` & `Some` values. This attribute supports multiple values separated by comma.

You can also use the `#[skip]` attribute to filter by specific fields:

```rust
use skippable_partialeq::SkippablePartialEq;

#[derive(Debug, SkippablePartialEq)]
pub struct Post {
    pub id: i64,
    pub content: String,
    pub author: i32,
    #[skip]
    pub creation_date: DateTime<Utc>,
}
```

This would exclude `creation_date` from the comparison.
