extern crate bencode;
extern crate hyper;
extern crate regex;
extern crate urlencoding;

use std::env;

mod metainfo;
mod tracker;
mod hash;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("{:?}", metainfo::from_file(filename));
}
