use std::collections::HashMap;

fn distance(c: char, i: usize, score: &Vec<(char, usize)>) -> usize {

	for idx in 0..score.len() {
		let (cur, _) = score[idx];
		if cur == c {
			if i > idx {
				return i - idx;
			} else {
				return idx - i;
			}
		}
	}

	score.len()
}

pub fn generate_frequency_map(target: &str) -> HashMap<char, usize> {
	let mut omap = HashMap::<char, usize>::new();

	for c in target.chars() {
		let new = omap.get(&c).unwrap_or(&0) + 1;
		omap.insert(c, new);
	}

	omap
}

fn sort_frequencies(map: &HashMap<char, usize>) -> Vec<(char, usize)> {
	let mut as_vec: Vec<(char, usize)> = map.iter().map(|(&x, &y)| (x, y)).collect();
	as_vec.sort_by(|&(idx, val), &(idx2, val2)| val.cmp(&val2));
	as_vec
}

pub fn score_string(target: &str) -> i64 {

	let mut omap = generate_frequency_map(target);
	let as_vec = sort_frequencies(&omap);
	
	let mut score: i64 = 100000;

	score -= 10 * distance('e', 0, &as_vec) as i64;
	score -= distance('t', 1, &as_vec) as i64;
	score -= distance('a', 2, &as_vec) as i64;
	score -= distance('o', 3, &as_vec) as i64;
	score -= distance('i', 4, &as_vec) as i64;
	score -= distance('n', 5, &as_vec) as i64;
	score -= distance('s', 6, &as_vec) as i64;
	score -= distance('r', 7, &as_vec) as i64;
	score -= distance('h', 8, &as_vec) as i64;
	score -= distance('d', 9, &as_vec) as i64;
	score -= distance('l', 10, &as_vec) as i64;
	score -= distance('u', 11, &as_vec) as i64;
	score -= distance('c', 12, &as_vec) as i64;
	score -= distance('m', 13, &as_vec) as i64;
	score -= distance('f', 14, &as_vec) as i64;
	score -= distance('y', 15, &as_vec) as i64;
	score -= distance('w', 16, &as_vec) as i64;
	score -= distance('g', 17, &as_vec) as i64;
	score -= distance('p', 18, &as_vec) as i64;
	score -= distance('b', 19, &as_vec) as i64;
	score -= distance('v', 20, &as_vec) as i64;
	score -= distance('k', 21, &as_vec) as i64;
	score -= distance('x', 22, &as_vec) as i64;
	score -= distance('q', 23, &as_vec) as i64;
	score -= distance('j', 24, &as_vec) as i64;
	score -= distance('z', 25, &as_vec) as i64;

	for (c, _) in as_vec {

		if c.is_lowercase() {
			score += 100;
		}

		if !c.is_alphabetic() {
			score -= 500;
		}
		if c.is_uppercase() {
			score -= 100;
		}
	}

	score as i64
}