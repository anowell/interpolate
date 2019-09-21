#![crate_type = "proc-macro"]
//! A simple form of Rust string interpolation
//!
//! `interpolate` provides basic string interpolation
//! functionality with a very light-weight syntax.
//!
//! ```no_run
//! #![feature(use_extern_macros, prox_macro_non_items)]
//! use interpolate::s;
//!
//! let name = "Hercules";
//! let greet = s!("Hello, {name}");
//! let sos = s!("HELP, {name.to_uppercase()}");
//! ```
//!
//! That is all.

#![feature(proc_macro_hygiene, proc_macro_quote)]

extern crate proc_macro;

use proc_macro::{quote, TokenStream, TokenTree};
use std::str::FromStr;

// Simple state machine for the current part of string being processed
#[derive(Debug, Copy, Clone)]
enum Position {
    // Any literal text that precedes a '$' interpolation
    Literal,
    // The delimeter(s) starting the interpolation, e.g., `{`
    ExpressionDelim,
    // An interpolation expression wrapped in braces, e.g., `{foo}`
    ExpressionWrapped,
    // Everything following the interpolation expression
    // (which may or may not contain additional expressions to interpolate)
    Rest,
}

#[derive(Default)]
struct FmtArgs {
    fmt: String,
    args: Vec<proc_macro::TokenStream>,
}
impl FmtArgs {
    fn new() -> FmtArgs {
        FmtArgs::default()
    }
}

// Splits the text into 3 parts:
// literal, interpolated expression, and remainder of string slice
// any one of these pieces may be empty:
//   if literal is empty, the text starts with an expression
//   if expression is empty, there are no expressions
//   if remainder is empty, then we've reached the end of the string
#[allow(unstable_name_collisions)]
fn split_interpolate(text: &str) -> FmtArgs {
    let mut state = Position::Literal;

    // Walk the string finding indexes for:
    //   delim: the first '$' delimeter
    //   exp_start: start of the interpolation expression (without delimeters)
    //   exp_end: the index immediately after the last char of the expression
    //   rest: the index of the first character of the rest of the string
    let mut fmt_args = FmtArgs::new();
    let (mut lit_start, mut delim, mut exp_start, mut exp_end) =
        (0, text.len(), text.len(), text.len());

    for (i, c) in text.char_indices() {
        match state {
            Position::Literal if c == '{' => {
                state = Position::ExpressionDelim;
                delim = i;
            }
            Position::ExpressionDelim if c == '{' => {
                state = Position::Literal;
                delim = text.len();
            }
            Position::ExpressionDelim => {
                state = Position::ExpressionWrapped;
                exp_start = i;
            }
            Position::ExpressionWrapped if c == '}' => {
                state = Position::Rest;
                exp_end = i;
            }
            Position::Rest if c == '}' => {
                state = Position::ExpressionWrapped;
                exp_end = text.len();
            }
            Position::Rest => {
                let (lit, arg) = unsafe {
                    (
                        text.get_unchecked(lit_start..delim),
                        text.get_unchecked(exp_start..exp_end)
                            .trim()
                            .replace("{{", "{")
                            .replace("}}", "}"),
                    )
                };
                fmt_args.fmt.push_str(lit);
                fmt_args.fmt.push_str("{}");
                fmt_args.args.push(
                    proc_macro::TokenStream::from_str(&arg)
                        .expect("interpolation expression is not a valid token stream"),
                );
                lit_start = i;
                if c == '{' {
                    state = Position::ExpressionDelim;
                    delim = i;
                } else {
                    state = Position::Literal;
                    delim = text.len();
                }
            }
            _ => (),
        }
    }

    match state {
        Position::ExpressionWrapped | Position::ExpressionDelim => {
            panic!("Interpolated expression is not closed")
        }
        Position::Rest => {
            let (lit, arg) = unsafe {
                (
                    text.get_unchecked(lit_start..delim),
                    text.get_unchecked(exp_start..exp_end)
                        .trim()
                        .replace("{{", "{")
                        .replace("}}", "}"),
                )
            };
            fmt_args.fmt.push_str(lit);
            fmt_args.fmt.push_str("{}");
            fmt_args.args.push(
                proc_macro::TokenStream::from_str(&arg)
                    .expect("interpolation expression is not a valid token stream"),
            );
        }
        Position::Literal => {
            let lit = unsafe { text.get_unchecked(lit_start..) };
            fmt_args.fmt.push_str(lit);
        }
    }
    // println!("FMT: {:?}", fmt_args.fmt);
    // for arg in &fmt_args.args {
    //   println!("  ARG: {:?}", arg);
    // }
    fmt_args
}

fn parse_args(input: TokenStream) -> FmtArgs {
    let mut trees = input.into_iter();
    let tree = trees
        .next()
        .expect("macro only accepts a single string literal");
    if let Some(_) = trees.next() {
        panic!("macro only accepts a single string literal");
    }
    // TODO: panic if multiple tokens
    let text = match tree {
        TokenTree::Literal(lit) => lit.to_string(),
        _ => panic!("macro only accepts a single string literal"),
    };

    let first_quote = text
        .find('"')
        .expect("macro only accepts a single string literal");
    if first_quote != 0 {
        panic!("macro does not accept raw string literals");
    }
    let text = unsafe { text.get_unchecked(1..(text.len() - 1)) };

    split_interpolate(&text)
}

/// Inline interpolation macro
#[proc_macro]
#[allow(unused_variables)]
pub fn s(input: TokenStream) -> TokenStream {
    let FmtArgs { fmt, args } = parse_args(input);
    quote!({ format!(#fmt, #(#args),*) })
}

/// Inline interpolating printing macro
#[proc_macro]
#[allow(unused_variables)]
pub fn p(input: TokenStream) -> TokenStream {
    let FmtArgs { fmt, args } = parse_args(input);
    quote!({ println!(#fmt, #(#args),*) })
}
