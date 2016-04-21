extern crate rustc_serialize;
mod xor;

use rustc_serialize::hex::FromHex;
use rustc_serialize::hex::ToHex;

fn main() {
	match std::env::args().nth(0) {
    	"xor" => { println!("{}", xor::xor(std::env::args().nth(2).unwrap().from_hex().unwrap(), std::env::args().nth(3).unwrap().from_hex().unwrap()).to_hex()); }
		_ => { println!("No selection"); }
	}
}
