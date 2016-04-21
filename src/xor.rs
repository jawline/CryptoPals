use string_score::*;

pub fn xor(origin: Vec<u8>, other: Vec<u8>) -> Vec<u8> {
	origin.iter().zip(other.iter()).map(|(x,y)| x ^ y).collect()
}

pub fn xor_all(origin: &Vec<u8>, v: u8) -> Vec<u8> {
	origin.iter().map(|x| x ^ v).collect()
}

pub fn one_byte_xor(origin: Vec<u8>) -> Result<String, String> {

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
	Err("Could not find any valid strings".to_string())
}