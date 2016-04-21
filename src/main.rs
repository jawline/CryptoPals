extern crate rustc_serialize;
mod xor;
mod hamming;
mod string_score;

use rustc_serialize::hex::FromHex;
use rustc_serialize::hex::ToHex;


use rustc_serialize::base64::FromBase64;
use rustc_serialize::base64::ToBase64;
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

fn buffer_file(file: &str) -> String {
	let mut f = File::open(file).unwrap();
	let mut buffer = String::new();
	f.read_to_string(&mut buffer);
	buffer
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
    			Ok((x, _)) => {
    				println!("Likely string {}", x.trim());
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
    				println!("Likely: {}", likely.trim());
    			},
    			Err(x) => {
    				println!("Error: {}", x);
    			}
    		}
    	},
    	Some(ref x) if x == "repeating_key_xor" => {
    		let text = std::env::args().nth(2).unwrap();
    		let key = std::env::args().nth(3).unwrap();
    		println!("{}", xor::repeating_key_xor(&text, &key).to_hex());
    	},
    	Some(ref x) if x == "hamming" => {
    		let s1 = std::env::args().nth(2).unwrap();
    		let s2 = std::env::args().nth(3).unwrap();
    		println!("Hamming {} and {}", s1, s2);
    		println!("{}", hamming::distance(&s1.into_bytes(), &s2.into_bytes()).unwrap());
    	},
    	Some(ref x) if x == "break_repeating_key" => {
    		let in_file = std::env::args().nth(2).unwrap();
    		println!("{}", xor::break_repeating_key(buffer_file(&in_file).from_base64().unwrap()));
    	},
		Some(ref x) => {
			println!("No selection {}", x);
		}
		_ => { println!("Incorrect number of arguments"); }
	}
}
