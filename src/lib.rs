#![feature(proc_macro)]
#![crate_type = "proc-macro"]

extern crate proc_macro;

use std::str::FromStr;
use std::iter::FromIterator;
use proc_macro::{TokenStream, TokenNode, Literal, quote};

#[proc_macro]
pub fn s(input: TokenStream) -> TokenStream {
    let tree = input.into_iter()
        .next().expect("macro only accepts a single string literal");

    // TODO: panic if multiple tokens

    let text = match tree.kind {
       TokenNode::Literal(lit) => lit.to_string(),
       _ => panic!("macro only accepts a single string literal"),
    };

    let mut tokens: Vec<TokenStream> = Vec::new();

    // FIXME: This assumes literal always starts and ends with single "
    // which is a flawed assumption
    let mut text = unsafe {
        text.get_unchecked(1..(text.len()-1))
    };

    tokens.push(quote!{
        let mut out = String::new();
    });

    loop {
       // println!("{}", text);
       let open = match text.find("${") {
           Some(i) => i,
           None => break,
       };
       let raw = Literal::string(unsafe { text.get_unchecked(0..open) });
       let raw_node = TokenNode::Literal(raw);
       tokens.push(quote! { out.push_str($raw_node); });

       text = unsafe { text.get_unchecked(open..) };
       let close = match text.find("}") {
          Some(i) => i,
          None => panic!("String interpolation missing closing '}' delimeter"),
       };

       let exp = unsafe { text.get_unchecked(2..close) };
       let exp = TokenStream::from_str(exp)
           .expect("Interpolated block is not a valid expression");

       tokens.push(quote! { out.push_str(&$exp.to_string()); });
       text = unsafe { text.get_unchecked((close+1)..) };
    }

    tokens.push( quote!{ out } );
    let stream = TokenStream::from_iter(tokens);
    quote!({ $stream })
}

#[proc_macro]
pub fn sprintln(input: TokenStream) -> TokenStream {
    let interpolated = s(input);

    // TODO: figure out 'Could not find `std` in `{{root}}`' error
    quote! {
        // println!("{}", $interpolated);
    }
}

