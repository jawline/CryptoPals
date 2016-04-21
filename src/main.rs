extern crate rustc_serialize;
mod xor;

use rustc_serialize::hex::FromHex;
use rustc_serialize::hex::ToHex;

fn main() {
	match std::env::args().nth(1) {
    	Some(ref x) if x == "xor" => {
    		let arg_one = std::env::args().nth(2).unwrap().from_hex().unwrap();
    		let arg_two = std::env::args().nth(3).unwrap().from_hex().unwrap();
    		println!("{}", xor::xor(arg_one, arg_two).to_hex());
    	},
		Some(ref x) => {
			println!("No selection {}", x);
		}
		_ => { println!("Incorrect number of arguments"); }
	}
}
