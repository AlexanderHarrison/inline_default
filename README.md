# Inline Default

A macro for implementing `Default` within a struct definition.
For example,
```rust
use inline_default::inline_default;

inline_default! {
struct KeyMap {
    left:   char = 'h',
    down:   char = 'j',
    up:     char = 'k',
    right:  char = 'l',
    flag:   bool, // uses bool::default() 
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
}

impl Default for KeyMap {
    fn default() {
        KeyMap {
            left:  'h',
            down:  'j',
            up:    'k',
            right: 'l',
            flag:  bool::default(),
        }
    }
}
```

## Supports:
- Visibility specifiers
- Use Default if not specified 
- Attributes (including derives)
- Lifetimes
- Generics, with major caveats

Due to the complexity in parsing trait bounds, 
only a single trait bound without generics is accepted.
`where` clauses are not supported.
Specifying lifetimes are accepted, but not the 'outlives' syntax `'a: 'b`.
For example,
```rust
use inline_default::inline_default;

inline_default! {
#[derive(Copy, Clone)]
pub(crate) struct Example<'a, T: Default> {
    pub a: &'a str = "example",
    b: T,
}}

```
is accepted, but the following are not:
```rust
use inline_default::inline_default;

// NOT VALID
inline_default! {
struct Example1<T: Default + Copy> {
    a: T,
}}

// NOT VALID
inline_default! {
struct Example2<T: From<f32>> {
    a: T = T::from(0.0),
}}

// NOT VALID
inline_default! {
struct Example2<'a, 'b: 'a> {
    a: &'a str = "test",
    b: &'b str = "test2",
}}
```

## Proc Macro?
Making a proc macro would be fuller feature-wise, but I couldn't be bothered.
