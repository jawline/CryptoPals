pub fn score_char(tc: char) -> usize {
	match tc.to_lowercase().next() {
		Some(x) if match x {
					'a' | 'e' | 'i' | 'o' | 'u' => true,
					_ => false 
				} => 10,
		_ => 0
	}
}

pub fn score_string(target: &str) -> usize {
	target.chars().fold(0, |old, x| old + score_char(x))
}