extern crate rustc_serialize;
mod xor;

use rustc_serialize::hex::FromHex;
use rustc_serialize::hex::ToHex;

fn main() {
    println!("{}", xor::xor(std::env::args().nth(1).unwrap().from_hex().unwrap(), std::env::args().nth(2).unwrap().from_hex().unwrap()).to_hex());
}
