fn byte_distance(first: u8, second: u8) -> usize {
	let mut dist = 0;
	let mut val = first ^ second;

	while val != 0 {
		dist += 1;
		val &= val - 1;
	}

	dist
}

pub fn distance(first: &str, second: &str) -> Result<usize, String> {
	if first.len() == second.len() {
		let s1 = first.to_string().into_bytes();
		let s2 = second.to_string().into_bytes();
		let distance = s1.iter().zip(s2.iter()).fold(0, |old, (x, y)| old + byte_distance(*x, *y));
		Ok(distance)
	} else {
		Err("Strings of different length".to_string())
	}
}