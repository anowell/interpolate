Interpolate
==========

A simple form of Rust string interpolation, 

Simple interpolation with format string literal, e.g., `f"Today is {date}"`.

[Documentation](http://docs.rs/interpolate)

[![Crates.io](https://img.shields.io/crates/v/interpolate.svg?maxAge=2592000)](https://crates.io/crates/interpolate)


## Usage


```rust
use interpolate::fstring;

#[fstring]
fn foo() {
    let name = "Hercules";
    let greet = f"Hello, {name}";
}
```

- Starting with Edition 2021, the prefix literal is reserved syntax, so you have to add a space `f "Hello {name}"`.
- The implementation simply changes `f"literal"` into `format!("literal")`.
- The `[fstring]` annotation is needed on an [item](https://doc.rust-lang.org/reference/items.html) because this is implemented as an [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros).


## Idea

The goal of interpolate was to explore basic string interpolation functionality with a very light-weight syntax.

I created this after a working on a CLI tools where I used `format!` a LOT.
I really wanted something lighter weight like Scala's `s"Today is $date"`, so
I decided to experiment here.

Originally it used `s!("Hi, {name}")`. To my delight, [Rust 1.58](https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html) started capturing identifiers in format strings, which basically meant this crate could be implemented in 2 lines:

```
pub use std::format as s;
pub use std::println as p;
```

So I started experimenting with prefix literal syntax.


Reference Material:
- [allowing literals to be used as String](https://internals.rust-lang.org/t/pre-rfc-allowing-string-literals-to-be-either-static-str-or-string-similar-to-numeric-literals/5029)
- [custom string literals](https://internals.rust-lang.org/t/pre-rfc-custom-string-literals/5037).
- [RFC 3101 - Reserved Literal Prefixes](https://github.com/rust-lang/rfcs/blob/master/text/3101-reserved_prefixes.md)
