<h1 align="center">Async Static</h1>
<div align="center">
 <strong>
    A macro for declaring async lazily evaluated statics in Rust.
 </strong>
</div>
<br />
<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/async_static">
    <img src="https://img.shields.io/crates/v/async_static.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/async_static">
    <img src="https://img.shields.io/crates/d/async_static.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/async_static">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- ci -->
  <a href="https://docs.rs/async_static">
    <img src="https://github.com/liangyongrui/async_static/workflows/Rust/badge.svg"
      alt="ci" />
  </a>
</div>

<br/>

## Basic usage

```rust
use async_static::async_static;

async fn get_num() -> i32 {
    println!("hello world");
    sleep(Duration::from_millis(100)).await;
    123
}

async_static! {
    static ref FOO:i32 = get_num().await;
}

/// run print
/// ```
/// hello world
/// The result of the first call: 123
/// The result of the second call: 123
/// ```
#[tokio::test]
async fn test() {
    // The first call, print hello world
    let n = FOO.await;
    println!("The result of the first call: {}", n);

    // The second call, nothing print
    let n = FOO.await;
    println!("The result of the second call: {}", n);
}
```

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions
