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

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use] extern crate quote;

use proc_macro::{TokenStream, TokenTree};
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

// Splits the text into 3 parts:
// literal, interpolated expression, and remainder of string slice
// any one of these pieces may be empty:
//   if literal is empty, the text starts with an expression
//   if expression is empty, there are no expressions
//   if remainder is empty, then we've reached the end of the string
#[allow(unstable_name_collisions)]
fn split_interpolate(text: &str) -> (String, String, &str) {
    let mut state = Position::Literal;

    // Walk the string finding indexes for:
    //   delim: the first '$' delimeter
    //   exp_start: start of the interpolation expression (without delimeters)
    //   exp_end: the index immediately after the last char of the expression
    //   rest: the index of the first character of the rest of the string
    let (mut delim, mut exp_start, mut exp_end, mut rest)
        = (text.len(), text.len(), text.len(), text.len());
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
                rest = i;
                break;
            }
            _ => (),
        }
    }

    match state {
        Position::ExpressionWrapped => panic!("Interpolated expression is not closed"),
        _ => (),
    }

    unsafe {(
        text.get_unchecked(..delim).replace("{{", "{").replace("}}", "}"),
        text.get_unchecked(exp_start..exp_end).replace("{{", "{").replace("}}", "}"),
        text.get_unchecked(rest..),
    )}
}

/// Inline interpolation macro
#[proc_macro]
pub fn s(input: TokenStream) -> TokenStream {
    let mut trees = input.into_iter();
    let tree = trees.next().expect("macro only accepts a single string literal");
    if let Some(_) = trees.next() {
        panic!("macro only accepts a single string literal");
    }
    // TODO: panic if multiple tokens

    let text = match tree {
       TokenTree::Literal(lit) => lit.to_string(),
       _ => panic!("macro only accepts a single string literal"),
    };

    let mut tokens = proc_macro2::TokenStream::new();

    let first_quote = text.find('"')
        .expect("macro only accepts a single string literal");
    if first_quote != 0 {
        panic!("macro does not accept raw string literals");
    }
    let mut text = unsafe { text.get_unchecked(1..(text.len()-1)) };

    tokens.extend(quote!{
        let mut out = String::new();
    });

    loop {
       //println!("ITER: {}", text);
       let (raw_lit, raw_exp, rest) = split_interpolate(&text);

       // Process any literal
       if raw_lit.len() > 0 {
           //println!("raw_lit: {}", raw_lit);
           tokens.extend(quote! { out.push_str(#raw_lit); });
       }

       // Process any interpolation expression
       if raw_exp.len() > 0 {
           //println!("raw_exp: {:?}", raw_exp);
           let exp = proc_macro2::TokenStream::from_str(&raw_exp).expect("not a valid token stream");
           tokens.extend(quote! { out.push_str(&#exp.to_string()); });
       }

       // Process the rest of the string (if any)
       if rest.len() == 0 {
           break;
       }

       text = rest;
    }

    // Return the `out` variable initialized and wrap the entire token stream in a block
    tokens.extend( quote!{ out } );
    //let stream = TokenStream::from_iter(tokens);
    quote!({ #tokens }).into()
}

