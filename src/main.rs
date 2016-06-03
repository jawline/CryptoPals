extern crate rustc_serialize;
extern crate openssl;
extern crate rand;

mod xor;
mod hamming;
mod string_score;
mod lssl;
mod random;
mod padding;

use rustc_serialize::hex::FromHex;
use rustc_serialize::hex::ToHex;

use rustc_serialize::base64::FromBase64;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::STANDARD;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn load_strings_hex(file: &str) -> Vec<Vec<u8>> {
	let mut result = Vec::new();

	let f = File::open(file).unwrap();
	let f = BufReader::new(f);

	for line in f.lines() {
		result.push(line.unwrap().from_hex().unwrap());
	}

	result
}

fn load_strings_base64(file: &str) -> Vec<Vec<u8>> {
    let mut result = Vec::new();

    let f = File::open(file).unwrap();
    let f = BufReader::new(f);

    for line in f.lines() {
        result.push(line.unwrap().from_base64().unwrap());
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
    		match xor::find_sbxor(load_strings_hex(&arg_one)) {
    			Ok((likely, _)) => {
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
    		println!("{}", xor::repeating_key_xor(&text.into_bytes(), &key.into_bytes()).to_hex());
    	},
    	Some(ref x) if x == "hamming" => {
    		let s1 = std::env::args().nth(2).unwrap();
    		let s2 = std::env::args().nth(3).unwrap();
    		println!("Hamming {} and {}", s1, s2);
    		println!("{}", hamming::distance(&s1.into_bytes(), &s2.into_bytes()).unwrap());
    	},
        Some(ref x) if x == "openssl_ecb_en" => {
            let s1 = std::env::args().nth(2).unwrap();
            let s2 = std::env::args().nth(3).unwrap();
            println!("Possible: {}", lssl::find_english(&load_strings_base64(&s1), &s2.into_bytes()).trim());
        },
        Some(ref x) if x == "guess_ecb" => {
            let s1 = std::env::args().nth(2).unwrap();
            println!("Possible: {}", lssl::guess_ecb(&load_strings_base64(&s1)));
        },
    	Some(ref x) if x == "break_repeating_key" => {
    		let in_file = std::env::args().nth(2).unwrap();
    		println!("{}", xor::break_repeating_key(buffer_file(&in_file).from_base64().unwrap()));
    	},
        Some(ref x) if x == "cbc_decrypt" => {
            let in_file = std::env::args().nth(2).unwrap();
            let key = std::env::args().nth(3).unwrap();
            println!("{}", String::from_utf8(lssl::cbc_decrypt(&buffer_file(&in_file).from_base64().unwrap(), 16, &key.into_bytes())).unwrap());
        },
        Some(ref x) if x == "pad_to_20" => {
            let in_text = std::env::args().nth(2).unwrap();
            println!("{}", String::from_utf8(padding::pad_block(&in_text.into_bytes(), 20, 'a' as u8)).unwrap());
        },
        Some(ref x) if x == "random_aes_key" => {
            println!("{}", random::generate_key(16).to_base64(STANDARD));
        },
		Some(ref x) => {
			println!("No selection {}", x);
		}
		_ => { println!("Incorrect number of arguments"); }
	}
}
