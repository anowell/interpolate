Interpolate
==========

A simple form of Rust string interpolation, e.g., `s!("Today is $date")`.

The goal of interpolate is to provide basic string interpolation
functionality with a very light-weight syntax.

It is not:

- A replacement for `format!`, `println!`, and related macros
- Capable of non-trivial formatting of types
- Anything that requires extensive documentation

Honestly, I created this after a working on a CLI tools where I used `format!` a LOT.
I really wanted something lighter weight, closer to Scala's `s"Today is $date"`,
so I decided to experiment here, with the idea of possibly turning any learnings
into a Pre-RFC for further discussion. After trying to use this, I found that
the indirection of hiding rust idents and expressions in a string literal made
formatting bad and errors less intuitive. Since it's less featureful than
`format!`, I found my particular code switching between the two styles more than expected.
And finally, a fair bit of the "lighter-weight" feeling could be achieved by just
aliasing `format!` to `s!`. So for now, I'm sorta setting this idea aside until/unless
I discover a more compelling use or direction it, but if you use or like it,
jot me a note.

## Usage

```rust

let name = "Jane";
let fav_num = 32;
let greeting = s!("$name's favorite number is $fav_num);
```

