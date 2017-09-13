#![feature(proc_macro)]
#![feature(rustc_private)]
#![crate_type = "proc-macro"]

//! A simple form of Rust string interpolation
//!
//! `interpolate` provides basic string interpolation
//! functionality with a very light-weight syntax.
//!
//! Note: `interpolate` currently requires experimental proc_macro functionality in nightly.
//!
//! ```no_run
//! #![feature(proc_macro)]
//! use interpolate::s;
//!
//! let name = "Hercules";
//! let greet = s!("Hello, $name");
//! let sos = s!("HELP, ${name.to_uppercase()}");
//! ```
//!
//! That is all.

extern crate proc_macro;

use std::str::FromStr;
use std::iter::FromIterator;
use proc_macro::{TokenStream, TokenNode, Literal, quote};

// Simple state machine for the current part of string being processed
#[derive(Debug, Copy, Clone)]
enum Position {
    // Any literal text that precedes a '$' interpolation
    Literal,
    // The delimeter(s) starting the interpolation, e.g., `$` or `${`
    ExpressionDelim,
    // An interpolation expression wrapped in braces, e.g., `${foo}`
    ExpressionWrapped,
    // An interpolation expression without the braces, e.g., `$foo`
    ExpressionBare,
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
fn split_interpolate(text: &str) -> (&str, &str, &str) {
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
            Position::Literal if c == '$' => {
                state = Position::ExpressionDelim;
                delim = i;
            }
            Position::ExpressionDelim if c == '{' => {
                state = Position::ExpressionWrapped;
                exp_start =  i + 1;
            }
            Position::ExpressionDelim if !c.is_xid_start() && c != '_' => {
                let msg = format!("Expected valid identifier or expression. Found '${}'.", c);
                panic!(msg);
            }
            Position::ExpressionDelim => {
                state = Position::ExpressionBare;
                exp_start = i;
            }
            Position::ExpressionWrapped if c == '}' => {
                state = Position::Rest;
                exp_end = i;
                rest = i + 1;
                break;
            }
            Position::ExpressionBare if !c.is_xid_continue() => {
                state = Position::Rest;
                exp_end = i;
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
        text.get_unchecked(..delim),
        text.get_unchecked(exp_start..exp_end),
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

    let text = match tree.kind {
       TokenNode::Literal(lit) => lit.to_string(),
       _ => panic!("macro only accepts a single string literal"),
    };

    let mut tokens: Vec<TokenStream> = Vec::new();

    let first_quote = text.find('"')
        .expect("macro only accepts a single string literal");
    if first_quote != 0 {
        panic!("macro does not accest raw string literals");
    }
    let mut text = unsafe { text.get_unchecked(1..(text.len()-1)) };

    tokens.push(quote!{
        let mut out = String::new();
    });

    loop {
       // println!("ITER: {}", text);
       let (raw_lit, raw_exp, rest) = split_interpolate(&text);

       // Process any literal
       if raw_lit.len() > 0 {
           let lit = TokenNode::Literal(Literal::string(raw_lit));
           tokens.push(quote! { out.push_str($lit); });
       }

       // Process any interpolation expression
       if raw_exp.len() > 0 {
           let exp = TokenStream::from_str(raw_exp)
               .expect("Interpolated block is not a valid expression");

           tokens.push(quote! { out.push_str(&$exp.to_string()); });
       }

       // Process the rest of the string (if any)
       if rest.len() == 0 {
           break;
       }

       text = rest;
    }

    // Return the `out` variable initialized and wrap the entire token stream in a block
    tokens.push( quote!{ out } );
    let stream = TokenStream::from_iter(tokens);
    quote!({ $stream })
}

