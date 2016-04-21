fn byte_distance(first: u8, second: u8) -> usize {
	let mut dist = 0;
	let mut val = first ^ second;

	while val != 0 {
		dist += 1;
		val = val & (val - 1);
	}

	dist
}

pub fn distance(first: &Vec<u8>, second: &Vec<u8>) -> Result<usize, String> {
	if first.len() == second.len() {
		let distance = first.iter().zip(second.iter()).fold(0, |old, (x, y)| old + byte_distance(*x, *y));
		Ok(distance)
	} else {
		Err("Strings of different length".to_string())
	}
}