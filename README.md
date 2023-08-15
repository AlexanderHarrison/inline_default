# Inline Default

[![Current Version](https://img.shields.io/crates/v/inline_default.svg)](https://crates.io/crates/inline_default)
[![License: MIT/Apache-2.0](https://img.shields.io/crates/l/inline_default.svg)]()

A macro for implementing `Default` within a struct definition.
This is useful when specifying the default values within the struct 
definition is cleaner than separating them, 
for example structs with a large number fields,
or when many structs are constructed.
For example,
```rust
use inline_default::inline_default;

inline_default! {
struct KeyMap {
    left:   char = 'h',
    down:   char = 'j',
    up:     char = 'k',
    right:  char = 'l',

    // uses bool::default(),
    flag:   bool, 
    
    // any constant-time expression is allowed
    qwerty: bool = Keyboard::IS_QWERTY,
}}
```
expands to

```rust
struct KeyMap {
    left:   char,
    down:   char,
    up:     char,
    right:  char,
    flag:   bool,
    qwerty: bool,
}

impl Default for KeyMap {
    fn default() {
        KeyMap {
            left:  'h',
            down:  'j',
            up:    'k',
            right: 'l',
            flag:  bool::default(),
            qwerty: Keyboard::IS_QWERTY,
        }
    }
}
```

## Supports:
- Visibility specifiers
- Use Default if not specified 
- Attributes (including derives) on both the struct and the fields
- Doc comments on both the struct and the fields
- Multiple struct definitions within each macro
- Lifetimes and generics, with major caveats

Due to the complexity in parsing trait bounds, 
only a single trait bound without generics is accepted.
`where` clauses are not supported.
Specifying lifetimes are accepted, but not the 'outlives' syntax `'a: 'b`.
For example, the following is accepted,
```rust
use inline_default::inline_default;

inline_default! {
/// Example struct
#[derive(Copy, Clone)]
pub(crate) struct Example<'a, T: Default> {
    pub a: &'a str = "example",

    /// field b
    b: T,
}}

```
but the following are not:
```rust
use inline_default::inline_default;

// NOT VALID - too many trait bounds on T
inline_default! {
struct Example1<T: Default + Copy> {
    a: T,
}}

// NOT VALID - Traits bounds cannot be generic
inline_default! {
struct Example2<T: From<f32>> {
    a: T = T::from(0.0),
}}

// NOT VALID - outlives syntax is not supported
inline_default! {
struct Example2<'a, 'b: 'a> {
    a: &'a str = "test",
    b: &'b str = "test2",
}}
```

## Proc Macro?
Making a proc macro would be fuller feature-wise, but I can't be bothered.
