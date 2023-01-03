use inline_default::inline_default;

type Key = char;

inline_default! {
#[derive(Copy, Clone)]
pub(crate) struct Example<'a, T: Default> {
    pub a: &'a str = "example",
    /// Doc comment (expands to #[doc = "..."] attribute).
    pub b: T,
}}

inline_default! {
#[derive(Debug, Clone)]
struct KeyMap {
    left:   Key = 'h',
    down:   Key = 'j',
    up:     Key = 'k',
    right:  Key = 'l',
}}

fn main() {
    let keymap: KeyMap = Default::default();
    println!("{:?}", keymap)
}
