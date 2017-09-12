#![feature(proc_macro)]

extern crate interpolate;
use interpolate::{s, sprintln};

fn main() {
    let name = "George";
    let age = 32;
    let msg = s!("Hello, ${name}. You are ${age}.");
    println!("Formatted: {}", msg);

    sprintln!("Goodbye, ${name}!");
}
