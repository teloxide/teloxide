# Code style

This is a description of a coding style that every contributor must follow.
Please, read the whole document before you start pushing code.

## Generics

All trait bounds should be written in `where`:

```rust
// GOOD
pub fn new<N, T, P, E>(user_id: i32, name: N, title: T, png_sticker: P, emojis: E) -> Self
where
    N: Into<String>,
    T: Into<String>,
    P: Into<InputFile>,
    E: Into<String>,
{ ... }

// BAD
pub fn new<N: Into<String>,
           T: Into<String>,
           P: Into<InputFile>,
           E: Into<String>>
    (user_id: i32, name: N, title: T, png_sticker: P, emojis: E) -> Self { ... }
```
```rust
// GOOD
impl<T> Trait for Wrap<T>
where
    T: Trait
{ ... }

// BAD
impl<T: Trait> Trait for Wrap<T> { ... }
```

**Rationale:**
- `where` clauses are easier to read when there are a lot of bounds
- uniformity

## Documentation comments

1. Documentation must describe _what_ your code does and mustn't describe _how_ your code does it and bla-bla-bla.
2. Be sure that your comments follow the grammar, including punctuation, the first capital letter and so on:
   ```rust
   // GOOD
   /// This function makes a request to Telegram.
   pub fn make_request(url: &str) -> String { ... }
   
   // BAD
   /// this function make request to telegram
   pub fn make_request(url: &str) -> String { ... }
   ```
3. Do not use ending punctuation in short list items (usually containing just one phrase or sentence):
   ```md
   <!-- GOOD -->
   - Handle different kinds of Update
   - Pass dependencies to handlers
   - Disable a default Ctrl-C handling

   <!-- BAD -->
   - Handle different kinds of Update.
   - Pass dependencies to handlers.
   - Disable a default Ctrl-C handling.

   <!-- BAD -->
   - Handle different kinds of Update;
   - Pass dependencies to handlers;
   - Disable a default Ctrl-C handling;
   ```
3. Link resources in your comments when possible:
   ```rust
   /// Download a file from Telegram.
   ///
   /// `path` can be obtained from the [`Bot::get_file`].
   ///
   /// To download into [`AsyncWrite`] (e.g. [`tokio::fs::File`]), see
   /// [`Bot::download_file`].
   ///
   /// [`Bot::get_file`]: crate::bot::Bot::get_file
   /// [`AsyncWrite`]: tokio::io::AsyncWrite
   /// [`tokio::fs::File`]: tokio::fs::File
   /// [`Bot::download_file`]: crate::Bot::download_file
   ```
4. Write `teloxide`, `teloxide-macros`, and `teloxide-core`, not "teloxide", "Teloxide", "teloxide-macros" or any other variant.

## Use `Self` where possible

When referring to the type for which block is implemented, prefer using `Self`, rather than the name of the type:

```rust
impl ErrorKind {
    // GOOD
    fn print(&self) {
        Self::Io => println!("Io"),
        Self::Network => println!("Network"),
        Self::Json => println!("Json"),
    }

    // BAD
    fn print(&self) {
        ErrorKind::Io => println!("Io"),
        ErrorKind::Network => println!("Network"),
        ErrorKind::Json => println!("Json"),
    }
}
```
```rust
impl<'a> AnswerCallbackQuery<'a> {
    // GOOD
    fn new<C>(bot: &'a Bot, callback_query_id: C) -> Self
    where
        C: Into<String>,
    { ... }

    // BAD
    fn new<C>(bot: &'a Bot, callback_query_id: C) -> AnswerCallbackQuery<'a>
    where
        C: Into<String>,
    { ... }
}
```

**Rationale:** `Self` is generally shorter and it's easier to copy-paste code or rename the type.

## Avoid duplication in fields names

```rust
struct Message {
    // GOOD
    #[serde(rename = "message_id")]
    id: MessageId,

    // BAD
    message_id: MessageId,
}
```

**Rationale:** duplication blurs the focus of code, making it unnecessarily longer.

## Conventional generic names

Use a generic parameter name `S` for streams, `Fut` for futures, `F` for functions (where possible).

**Rationale:** uniformity.

## Deriving traits

Derive `Copy`, `Clone`, `Eq`, `PartialEq`, `Hash` and `Debug` for public types when possible.

**Rationale:** these traits can be useful for users and can be implemented for most types.

Derive `Default` when there is a reasonable default value for the type.

**Rationale:** `Default` plays nicely with generic code (for example, `mem::take`).

## `Into`-polymorphism

Use `T: Into<Ty>` when this can simplify user code.
I.e. when there are types that implement `Into<Ty>` that are likely to be passed to this function.

**Rationale:** conversions unnecessarily complicate caller code and can be confusing for beginners.

## `must_use`

Always mark functions as `#[must_use]` if they don't have side effects and the only reason to call them is to get the result:

```rust
impl User {
    // GOOD
    #[must_use]
    fn full_name(&self) -> String {
        format!("{} {}", user.first_name, user.last_name)
    }
}
```

**Rationale:** users will get warnings if they forgot to do something with the result, potentially preventing bugs.

## Creating boxed futures

Prefer `Box::pin(async { ... })` instead of `async { ... }.boxed()`.

**Rationale:** the former is generally formatted better by rustfmt.

## Full paths for logging

Always write `log::<op>!(...)` instead of importing `use log::<op>;` and invoking `<op>!(...)`.

```rust
// GOOD
log::warn!("Everything is on fire");

// BAD
use log::warn;

warn!("Everything is on fire");
```

**Rationale:**
- Less polluted import blocks
- Uniformity

## `&str` -> `String` conversion

Prefer using `.to_owned()`, rather than `.to_string()`, `.into()`, `String::from`, etc.

**Rationale:** uniformity, intent clarity.

## Order of imports

Separate import groups with blank lines. Use one use per crate.

Module declarations come before the imports.
Order them in "suggested reading order" for a person new to the code base.

```rust
mod x;
mod y;

// First std.
use std::{ ... }

// Second, external crates (both crates.io crates and other rust-analyzer crates).
use crate_foo::{ ... }
use crate_bar::{ ... }

// Then current crate.
use crate::{}

// Finally, parent and child modules, but prefer `use crate::`.
use super::{}

// Re-exports are treated as item definitions rather than imports, so they go
// after imports and modules. Use them sparingly.
pub use crate::x::Z;
```

**Rationale:**
- Reading order is important for new contributors
- Grouping by crate allows spotting unwanted dependencies easier
- Consistency

## Import Style

When implementing traits from `std::fmt` import the module:

```rust
// GOOD
use std::fmt;

impl fmt::Display for RenameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { .. }
}

// BAD
impl std::fmt::Display for RenameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { .. }
}
```

**Rationale:**
- Makes it clear that a trait is implemented, rather than used
- Less typing

Prefer `use crate::foo::bar` to `use super::bar` or `use self::bar::baz`. **Rationale:**
- Works in all cases
- Consistency

## Order of Items

Optimize for the reader who sees the file for the first time, and wants to get a general idea about what's going on. People read things from top to bottom, so place most important things first.

Specifically, if all items except one are private, always put the non-private item on top:
```rust
// GOOD
pub(crate) fn frobnicate() {
    Helper::act()
}

#[derive(Default)]
struct Helper { stuff: i32 }

impl Helper {
    fn act(&self) {

    }
}

// BAD
#[derive(Default)]
struct Helper { stuff: i32 }

pub(crate) fn frobnicate() {
    Helper::act()
}

impl Helper {
    fn act(&self) {

    }
}
```

If there's a mixture of private and public items, put public items first.

Put structs and enums first, functions and impls last. Order type declarations in a top-down manner:

```rust
// GOOD
struct Parent {
    children: Vec<Child>
}

struct Child;

impl Parent {
}

impl Child {
}

// BAD
struct Child;

impl Child {
}

struct Parent {
    children: Vec<Child>
}

impl Parent {
}
```

**Rationale:**
- Easier to get a sense of the API by visually scanning the file
- If function bodies are folded in the editor, the source code should be read as documentation for the public API

## Early Returns

Do use early returns:

```rust
// GOOD
fn foo() -> Option<Bar> {
    if !condition() {
        return None;
    }

    Some(...)
}

// BAD
fn foo() -> Option<Bar> {
    if condition() {
        Some(...)
    } else {
        None
    }
}
```

**Rationale:** reduce cognitive stack usage.

## If-let

Avoid the `if let ... { } else { }` construct, use `match` instead:

```rust
// GOOD
match ctx.expected_type.as_ref() {
    Some(expected_type) => completion_ty == expected_type && !expected_type.is_unit(),
    None => false,
}

// BAD
if let Some(expected_type) = ctx.expected_type.as_ref() {
    completion_ty == expected_type && !expected_type.is_unit()
} else {
    false
}
```

**Rationale:**
- `match` is almost always more compact
- The `else` branch can get a more precise pattern: `None` or `Err(_)` instead of `_`

## Empty Match Arms

Use `=> (),` when a match arm is intentionally empty:
```rust
// GOOD
match result {
    Ok(_) => (),
    Err(err) => error!("{}", err),
}

// BAD
match result {
    Ok(_) => {}
    Err(err) => error!("{}", err),
}
```

**Rationale:** consistency.
