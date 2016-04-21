use string_score::*;
use hamming;
use std::cmp::Ordering;

pub fn xor(origin: Vec<u8>, other: Vec<u8>) -> Vec<u8> {
	origin.iter().zip(other.iter()).map(|(x,y)| x ^ y).collect()
}

pub fn xor_all(origin: &Vec<u8>, v: u8) -> Vec<u8> {
	origin.iter().map(|x| x ^ v).collect()
}

pub fn one_byte_xor(origin: &Vec<u8>) -> Result<(String, u8), String> {

	let mut highest = None;
	let mut highest_key = 0;
	let mut highest_score = -9999999;

	for xor_v in 0..255 {
		let xored = xor_all(&origin, xor_v);
		let decoded = String::from_utf8(xored);

		if decoded.is_ok() {
			let unwrapped = decoded.unwrap();
			let local_score = score_string(&unwrapped);
			if local_score > highest_score {
				highest = Some(unwrapped);
				highest_key = xor_v;
				highest_score = local_score;
			}
		}
	}

	if highest.is_some() {
		Ok((highest.unwrap(), highest_key))
	} else {
		Err("Could not find any valid strings".to_string())
	}
}

pub fn find_sbxor(potentials: Vec<Vec<u8>>) -> Result<(String, Vec<u8>), String> {
	let mut highest = None;
	let mut highest_score = 0;

	for potential in potentials {
		let xor_res = one_byte_xor(&potential);
		let is_ok = xor_res.is_ok();
		if is_ok {
			let (unwrapped, _) = xor_res.unwrap();
			let score = score_string(&unwrapped);
			if score > highest_score {
				highest = Some((unwrapped, potential.clone()));
				highest_score = score;
			}
		}
	}
	
	if highest.is_some() {
		Ok(highest.unwrap())
	} else {
		Err("Could not find any valid strings".to_string())
	}
}

fn handle_key_size(cipher: &Vec<u8>, key_size: usize) -> Vec<u8> {
	println!("Handling key {}", key_size);

	let mut blocks = Vec::new();
	let mut current = cipher.iter();

	for i in 0..cipher.len() / key_size {
		let block: Vec<u8> = cipher.iter().skip(i * key_size).take(key_size).map(|&x| x).collect();
		blocks.push(block);
	}

	println!("Constructed {} blocks", blocks.len());

	let mut transposed = Vec::new();

	for i in 0..key_size {
		let transposed_byte: Vec<u8> = blocks.iter().map(|x| x[i]).collect();
		transposed.push(transposed_byte);
	}

	println!("Transposed the bytes");

	let mut final_key = Vec::new();

	for i in 0..key_size {
		let (_, key) = one_byte_xor(&transposed[i]).unwrap();
		final_key.push(key);
	}

	final_key
}

pub fn break_repeating_key(cipher: Vec<u8>) -> String {

	let mut key_scores = Vec::new();

	for key_size in 2..40 {
		let block_one: Vec<u8> = cipher.iter().cloned().take(key_size).collect();
		let block_two: Vec<u8> = cipher.iter().cloned().skip(key_size).take(key_size).collect();
		key_scores.push((key_size, hamming::distance(&block_one, &block_two).unwrap() as f32 / key_size as f32));
	}

	key_scores.sort_by(|&(idx, val), &(idx2, val2)| val.partial_cmp(&val2).unwrap_or(Ordering::Equal));

	let mut valid_strings = Vec::new();

	for &(idx, val) in key_scores.iter().take(4) {
		let key = handle_key_size(&cipher, idx);
		valid_strings.push(String::from_utf8(repeating_key_xor(&cipher, &key)).unwrap());
	}

	for item in valid_strings {
		println!("String {}", item);
	}

	"Aaaah".to_string()
}


pub fn repeating_key_xor(text: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
	let mut result = Vec::new();

	for i in 0..text.len() {
		result.push(text[i] ^ key[i % key.len()]);
	}

	result
}