use string_score::*;
use hamming;

pub fn xor(origin: Vec<u8>, other: Vec<u8>) -> Vec<u8> {
	origin.iter().zip(other.iter()).map(|(x,y)| x ^ y).collect()
}

pub fn xor_all(origin: &Vec<u8>, v: u8) -> Vec<u8> {
	origin.iter().map(|x| x ^ v).collect()
}

pub fn one_byte_xor(origin: &Vec<u8>) -> Result<String, String> {

	let mut highest = None;
	let mut highest_score = 0;

	for xor_v in 0..255 {
		let xored = xor_all(&origin, xor_v);
		let decoded = String::from_utf8(xored);

		if decoded.is_ok() {
			let unwrapped = decoded.unwrap();
			let local_score = score_string(&unwrapped);
			if local_score > highest_score {
				highest = Some(unwrapped);
				highest_score = local_score;
			}
		}
	}

	if highest.is_some() {
		Ok(highest.unwrap())
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
			let unwrapped = xor_res.unwrap();
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

pub fn break_repeating_key(cipher: Vec<u8>) -> String {

	let mut key_scores = Vec::new();

	for key_size in 2..40 {
		let block_one: Vec<u8> = cipher.iter().cloned().take(key_size).collect();
		let block_two: Vec<u8> = cipher.iter().cloned().skip(key_size).take(key_size).collect();
		key_scores.push((key_size, hamming::distance(&block_one, &block_two).unwrap() / key_size));
	}

	key_scores.sort_by(|&(idx, val), &(idx2, val2)| val.cmp(&val2));

	for (idx, val) in key_scores {
		println!("{}: {}", idx, val);
	}

	"Aaaah".to_string()
}


pub fn repeating_key_xor(text: &str, key: &str) -> Vec<u8> {
	let text_bytes = text.to_string().into_bytes();
	let key = key.to_string().into_bytes();
	let mut result = Vec::new();

	for i in 0..text_bytes.len() {
		result.push(text_bytes[i] ^ key[i % key.len()]);
	}

	result
}