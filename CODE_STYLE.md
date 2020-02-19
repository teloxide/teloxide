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

## Comments
 1. Comments must describe what your code does and mustn't describe how your code does it and bla-bla-bla. Be sure that your comments follow the grammar, including punctuation, the first capital letter and so on.

Bad:

```rust
/// this function make request to telegram
pub fn make_request(url: &str) -> String { ... }
```

Good:

```rust
/// This function makes a request to Telegram.
pub fn make_request(url: &str) -> String { ... }
```

 2. Also, link resources in your comments when possible:

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
#[cfg(feature = "unstable-stream")]
pub async fn download_file_stream(
    &self,
    path: &str,
) -> Result<impl Stream<Item = Result<Bytes, reqwest::Error>>, reqwest::Error>
{
    download_file_stream(&self.client, &self.token, path).await
}
```

## Use Self where possible
Bad:

```rust
impl ErrorKind {
    fn print(&self) {
        ErrorKind::Io => println!("Io"),
        ErrorKind::Network => println!("Network"),
        ErrorKind::Json => println!("Json"),
    }
}
```

Good:
```rust
impl ErrorKind {
    fn print(&self) {
        Self::Io => println!("Io"),
        Self::Network => println!("Network"),
        Self::Json => println!("Json"),
    }
}
```

<details>
    <summary>More examples</summary>
    
Bad:
    
```rust
impl<'a> AnswerCallbackQuery<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, callback_query_id: C) -> AnswerCallbackQuery<'a>
    where
C: Into<String>, { ... }
```

Good:
    
```rust
impl<'a> AnswerCallbackQuery<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, callback_query_id: C) -> Self
    where
C: Into<String>, { ... }
```
</details>

## Naming
 1. Avoid unnecessary duplication (`Message::message_id` -> `Message::id` using `#[serde(rename = "message_id")]`).
 2. Use a generic parameter name `S` for streams, `Fut` for futures, `F` for functions (where possible).

## Deriving
 1. Derive `Copy`, `Eq`, `Hash`, `PartialEq`, `Clone`, `Debug` for public types when possible (note: if the default `Debug` implementation is weird, you should manually implement it by yourself).
 2. Derive `Default` when there is an algorithm to get a default value for your type.

## Misc
 1. Use `Into<...>` only where there exists at least one conversion **and** it will be logically to use.
 2. Always mark a function as `#[must_use]` if its return value **must** be used.
