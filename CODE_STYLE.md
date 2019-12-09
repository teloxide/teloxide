# Code style
This is a description of a coding style that every contributor must follow. Please, read the whole document before you start pushing code.

## Generics
Generics are always written with `where`.

Bad:

```rust
    pub fn new<N: Into<String>,
               T: Into<String>,
               P: Into<InputFile>,
               E: Into<String>>
    (user_id: i32, name: N, title: T, png_sticker: P, emojis: E) -> Self { ... }
```

Good:

```rust
    pub fn new<N, T, P, E>(user_id: i32, name: N, title: T, png_sticker: P, emojis: E) -> Self
    where
        N: Into<String>,
        T: Into<String>,
        P: Into<InputFile>,
        E: Into<String> { ... }
```
