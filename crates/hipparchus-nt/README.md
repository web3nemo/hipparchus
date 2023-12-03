See in https://github.com/web3nemo/hipparchus.

[![github]](https://github.com/web3nemo/hipparchus)&ensp;
[![crates-io]](https://crates.io/crates/hipparchus-wrap)&ensp;
[![docs-rs]](https://docs.rs/hipparchus-wrap)&ensp;

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

The crate provides some utiltiy to create wrapped newtype and implement forwarded operations and traits.

The [newtype](https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html) idiom is a design pattern in Rust that is responsible for ensuring that the right value type is provided to the program at compile time. The newtype patterns are used in scenarios of primitive type like number or string and they are wrapped inside a struct so that when the programmer debugs the code then the type checker in Rust could catch any type of errors and provide safety to our code and at the same time enhance our code readability. However, to implement the precluded operations & traits of primitive types for the wrapped newtype is a bit cumbersome and boring. Our goal is to embrace newtype idiom in convenient way.

Here is the features list:
- `Wrapped` trait and declarative macro `wrap!` to create the wrapped type with `Wrapped` trait implemented.
- A collection of declarative macros to implement various unary or binary operation (and related traits) easily.
  - `impl_wrapped_op_unary!`: Implement unary operator for wrapped type
  - `impl_wrapped_op_binary!`: Implement binary operator for wrapped type
  - `impl_wrapped_op_binary_mut!`: Implement binary operator for mutable wrapped type
  - `impl_wrapped_op_binarywith!`: Implement binary operator for wrapped type with non-wrapped rhs
  - `impl_wrapped_op_binarywith_mut!`: Implement binary operator for mutable wrapped type with non-wrapped rhs
- A collection of declarative macros to implement preinclude rust operations (and related traits) easily. 
  - `impl_wrapped_ops_arithmatic!`
  - `impl_wrapped_ops_bitwise!`
  - ...

# Usage

Add the following to your '''Cargo.toml''':

```toml
[dependencies]
hipparchus-wrap = "0.1"
```

# Examples

Here is a simple example to wrap a primitive type with arithmatic operation implemented.

``` rs
// TODO
```

# License

This project is licensed under either of
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](https://github.com/web3nemo/hipparchus/blob/HEAD/LICENSE-APACHE))
- [MIT License](https://opensource.org/license/mit/) ([LICENSE-MIT](https://github.com/rust-lang/libc/blob/HEAD/LICENSE-MIT))

at your option.

# Contributing

We welcome all people who want to contribute. Please see the contributing instructions for more information.

Contributions in any form (issues, pull requests, etc.) to this project must adhere to Rust's Code of Conduct.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in '''hipparchus-*''' by you, 
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
