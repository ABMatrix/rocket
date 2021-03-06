#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

use rocket::response::{content, Stream};

use std::io::{repeat, Repeat, Read, Take};
use std::fs::File;

type LimitedRepeat = Take<Repeat>;

// Generate this file using: head -c BYTES /dev/random > big_file.dat
const FILENAME: &str = "big_file.dat";

#[get("/")]
fn root() -> content::Plain<Stream<LimitedRepeat>> {
    content::Plain(Stream::from(repeat('a' as u8).take(25000)))
}

#[get("/big_file")]
fn file() -> Option<Stream<File>> {
    File::open(FILENAME).map(|file| Stream::from(file)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![root, file])
}

fn main() {
    rocket().launch();
}
