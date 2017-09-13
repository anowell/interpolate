Interpolate
==========

A simple form of Rust string interpolation, e.g., `s!("Today is $date")`.

The goal of interpolate is to provide basic string interpolation
functionality with a very light-weight syntax.

It is not:

- A replacement for `format!`, `println!`, and related macros
- Capable of non-trivial formatting of types
- Anything that requires extensive documentation


## Usage

```rust

let name = "Jane";
let fav_num = 32;
let greeting = s!("$name's favorite number is $fav_num);
```



