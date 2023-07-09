# derive(JsError)

Allows returning an error from a `wasm_bindgen` function without a separate API.

## Motivation

`wasm_bindgen` requires a function returning `Result` to either be `Result<T, JsError>`
or `Result<T, E> where E: Into<JsValue>`.
If you wish to return any other error type into Javascript,
the recommendation is to have an internal function that returns a normal `Result<T, E>`,
and to have a public API that wraps the function's return value into a `Result<T, JsError>`
(see example [here](https://docs.rs/wasm-bindgen/0.2.87/wasm_bindgen/struct.JsError.html#complex_example)
at `wasm_bindgen` documentation).
For a library with both a WASM build and a standard library build, however,
this could create a redundant API,
with multiple `#[wasm_bindgen]` options that only wrap the internal function.

An alternative solution is to trivially implement `Into<JsValue>` for the error type:

```rust
impl From<ErrorType> for JsValue {
    fn from(error: Error) -> Self {
        JsError::from(error).into()
    }
}
```

This crate simply wraps this code into a `#[derive(JsError)]` macro for convenience.

## Installation

Install with `cargo add`:

```sh
cargo add derive_jserror@0.1
```

Or by modifying `Cargo.toml`:

```toml
[dependencies]
derive_jserror = "0.1"
```

## Usage

Example with [`thiserror`](https://github.com/dtolnay/thiserror):

```rust
use derive_jserror::JsError;
use wasm_bindgen::prelude::*;
use thiserror::Error;

#[derive(Error, JsError, Debug)]
#[error("test error type")]
struct TestError;

#[wasm_bindgen]
pub fn func_returns_result() -> Result<(), TestError> {
    Err(TestError)
}
```

## License

Licensed under either of [Apache License (Version 2.0)](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution
intentionally submitted for inclusion in this crate by you,
as defined in the Apache-2.0 license,
shall be dual licensed as above,
without any additional terms or conditions.
