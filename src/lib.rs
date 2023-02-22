#![crate_type = "proc-macro"]
//! Simple interpolation with format string literal
//!
//! `interpolate` provides basic format string literal syntax
//! that explores the design space reserved by [RFC 3101](https://github.com/rust-lang/rfcs/blob/master/text/3101-reserved_prefixes.md)
//!
//! ```no_run
//! use interpolate::fstring;
//!
//! #[fstring]
//! fn foo() {
//!     let name = "Hercules";
//!     let greet = f"Hello, {name}";
//! }
//! ```
//!
//! - Starting with Edition 2021, the prefix literal is reserved syntax, so you have to add a space `f "Hello {name}"`.
//! - The implementation simply changes `f"literal"` into `format!("literal")`.
//! - The `[fstring]` annotation is needed on an [item](https://doc.rust-lang.org/reference/items.html) because this is implemented as an [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros).
//!
//! That is all.

extern crate proc_macro;
use std::iter::FromIterator;

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, TokenStream, TokenTree};

fn process_stream(item: TokenStream) -> TokenStream {
    let mut output = Vec::new();
    let mut format_ident: Option<Ident> = None;

    for tree in item {
        if let Some(f_ident) = format_ident {
            match tree.clone() {
                // Here's where we replace f"lit" with format!("lit")
                tree @ TokenTree::Literal(_) => {
                    // println!("Constructing format!({tree})");
                    output.push(TokenTree::from(Ident::new("format", f_ident.span())));
                    output.push(TokenTree::from(Punct::new('!', Spacing::Alone)));
                    let group = Group::new(Delimiter::Parenthesis, TokenStream::from(tree));
                    output.push(TokenTree::from(group));
                    format_ident = None;
                    continue;
                }
                _ => {
                    // println!("Found the format identifier without a literal.");
                    output.push(TokenTree::from(f_ident));
                }
            }
        }
        format_ident = None;

        match tree {
            TokenTree::Group(group) => {
                let new_stream = process_stream(group.stream());
                let new_group = Group::new(group.delimiter(), new_stream);
                output.push(TokenTree::from(new_group));
            }
            TokenTree::Ident(ident) => {
                // println!("Ident: {ident}");
                if ident.to_string() == "f" {
                    format_ident = Some(ident);
                } else {
                    output.push(TokenTree::from(ident));
                }
            }
            tree @ TokenTree::Punct(_) => output.push(tree),
            tree @ TokenTree::Literal(_) => output.push(tree),
        }
    }

    if let Some(f_ident) = format_ident {
        // println!("Found the format identifier without a literal at end of TokenStream.");
        output.push(TokenTree::from(f_ident));
    }

    TokenStream::from_iter(output)
}

#[proc_macro_attribute]
pub fn fstring(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let output = process_stream(item);
    // println!("{output}");
    output
}
