extern crate rustc_serialize;
mod xor;
mod string_score;

use rustc_serialize::hex::FromHex;
use rustc_serialize::hex::ToHex;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn load_strings(file: &str) -> Vec<Vec<u8>> {
	let mut result = Vec::new();

	let f = File::open(file).unwrap();
	let f = BufReader::new(f);

	for line in f.lines() {
		result.push(line.unwrap().from_hex().unwrap());
	}

	result
}

fn main() {
	match std::env::args().nth(1) {
    	Some(ref x) if x == "xor" => {
    		let arg_one = std::env::args().nth(2).unwrap().from_hex().unwrap();
    		let arg_two = std::env::args().nth(3).unwrap().from_hex().unwrap();
    		println!("{}", xor::xor(arg_one, arg_two).to_hex());
    	},
    	Some(ref x) if x == "sbxor" => {
    		let arg_one = std::env::args().nth(2).unwrap().from_hex().unwrap();
    		println!("Attempting to decrypt hex encoded argument {}", std::env::args().nth(2).unwrap());
    		match xor::one_byte_xor(&arg_one) {
    			Ok(x) => {
    				println!("Likely string {}", x);
    			},
    			Err(x) => {
    				println!("Error {}", x);
    			}
    		}
    	},
    	Some(ref x) if x == "find_sbxor" => {
    		let arg_one = std::env::args().nth(2).unwrap();
    		println!("Loading potential SBXOR's from file {}", arg_one);
    		match xor::find_sbxor(load_strings(&arg_one)) {
    			Ok((likely, bytes)) => {
    				println!("Likely {}", likely);
    			},
    			Err(x) => {
    				println!("Error {}", x);
    			}
    		}
    	},
		Some(ref x) => {
			println!("No selection {}", x);
		}
		_ => { println!("Incorrect number of arguments"); }
	}
}
