fn is_vowel(tc: char) -> bool {
	match tc.to_lowercase().next() {
		Some(x) if match x {
					'a' | 'e' | 'i' | 'o' | 'u' => true,
					_ => false 
				} => true,
		_ => false
	}
}

pub fn score_char(tc: char) -> i32 {

	let mut score = 0;

	if tc.is_digit(10) {
		score = 10;
	}

	if tc.is_alphabetic() {
		score = 25;
	}

	if is_vowel(tc) {
		score = 50;
	}

	if !tc.is_alphanumeric() {
		score = -50;
	}

	score
}

pub fn score_string(target: &str) -> i32 {
	target.chars().fold(0, |old, x| old + score_char(x))
}