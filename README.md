Interpolate
==========

A simpler form of Rust string interpolation, e.g., `s!("Today is $date")`.

The goal of interpolate is to provide basic string interpolation
functionality with a very light-weight syntax.

What it is not:

- A replacement for `format!`, `println!`, and friends
- Capable of non-trivial formatting of types
- Anything that requires extensive documentation


## Usage

The goal is to support `$foo` syntax, but currently
(for the sake of prototyping) it requires being enclosed
in braces, e.g., `${foo}`.

```rust

let name = "Jane";
let fav_num = 32;
let greeting = s!("$name's favorite number is $fav_num);
```

