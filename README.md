Interpolate
==========

A simple form of Rust string interpolation, e.g., `s!("Today is {date}")`.

[Documentation](http://docs.rs/interpolate)

[![Crates.io](https://img.shields.io/crates/v/interpolate.svg?maxAge=2592000)](https://crates.io/crates/interpolate)


## Usage

Note: `interpolate` currently requires some experimental functionality in nightly.

```rust
#![feature(use_extern_macros, proc_macro)]
use interpolate::s;

let name = "Jane";
let fav_num = 32;
let greeting = s!("{name}'s favorite number is {fav_num}");
```

Escaping braces is accomplished similar to escaping other format strings in rust.

> The literal characters { and } may be included in a string by preceding them with the same character. For example, the { character is escaped with {{ and the } character is escaped with }}.

## Idea

The goal of interpolate is to provide basic string interpolation functionality with a very light-weight syntax.

It is not:

- A full replacement for `format!`, `println!`, and related macros
- Capable of non-trivial formatting of types
- Anything that requires extensive documentation

I created this after a working on a CLI tools where I used `format!` a LOT.
I really wanted something lighter weight like Scala's `s"Today is $date"`, so
I decided to experiment here, with the idea of possibly adding to the
discussions around strings (like
[allowing literals to be used as String](https://internals.rust-lang.org/t/pre-rfc-allowing-string-literals-to-be-either-static-str-or-string-similar-to-numeric-literals/5029)
and [custom string literals](https://internals.rust-lang.org/t/pre-rfc-custom-string-literals/5037).
I frequently find myself wondering if any of these ideas could have a more central role in rust:

- `println!("Hello {name}")` to basically mean `println!("Hello {name}", name=name)`
- `let full_name = s"{first_name} {last_name}"` instead of `format!("{} {}", first_name, last_name)`
- `let msg = s"Hello"` instead of `"Hello".to_string()`
