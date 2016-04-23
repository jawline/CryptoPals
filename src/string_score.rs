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

fn score_distance(c: char, freq: f64, expected: f64) -> f64 {
	let res = (freq - expected) * 1000.0;
	-(res).abs()
}

pub fn generate_frequency_map(target: &str) -> (HashMap<char, usize>, usize) {
	let mut omap = HashMap::<char, usize>::new();
	let mut num_chars = 0;

	for c in target.chars() {
		let new = omap.get(&c).unwrap_or(&0) + 1;
		omap.insert(c, new);

		if c.is_alphabetic() {
			num_chars += 1;
		}
	}

	(omap, num_chars)
}

fn sort_frequencies(map: &HashMap<char, usize>) -> Vec<(char, usize)> {
	let mut as_vec: Vec<(char, usize)> = map.iter().map(|(&x, &y)| (x, y)).collect();
	as_vec.sort_by(|&(idx, val), &(idx2, val2)| val.cmp(&val2));
	as_vec
}

pub fn score_string(target: &str) -> f64 {

	let (mut omap, ccount) = generate_frequency_map(target);
	let as_vec = sort_frequencies(&omap);
	
	let mut score: f64 = 0.0;

	score += score_distance('a', *omap.get(&'a').unwrap_or(&0) as f64 / ccount as f64, 0.08);
	score += score_distance('b', *omap.get(&'b').unwrap_or(&0) as f64 / ccount as f64, 0.01);
	score += score_distance('c', *omap.get(&'c').unwrap_or(&0) as f64 / ccount as f64, 0.03);
	score += score_distance('d', *omap.get(&'d').unwrap_or(&0) as f64 / ccount as f64, 0.04);
	score += score_distance('e', *omap.get(&'e').unwrap_or(&0) as f64 / ccount as f64, 0.14);
	score += score_distance('f', *omap.get(&'f').unwrap_or(&0) as f64 / ccount as f64, 0.01);
	score += score_distance('g', *omap.get(&'g').unwrap_or(&0) as f64 / ccount as f64, 0.01);
	score += score_distance('h', *omap.get(&'h').unwrap_or(&0) as f64 / ccount as f64, 0.06);
	score += score_distance('i', *omap.get(&'i').unwrap_or(&0) as f64 / ccount as f64, 0.06);
	score += score_distance('j', *omap.get(&'j').unwrap_or(&0) as f64 / ccount as f64, 0.005);
	score += score_distance('k', *omap.get(&'k').unwrap_or(&0) as f64 / ccount as f64, 0.01);
	score += score_distance('l', *omap.get(&'l').unwrap_or(&0) as f64 / ccount as f64, 0.04);
	score += score_distance('m', *omap.get(&'m').unwrap_or(&0) as f64 / ccount as f64, 0.02);
	score += score_distance('n', *omap.get(&'n').unwrap_or(&0) as f64 / ccount as f64, 0.07);
	score += score_distance('o', *omap.get(&'o').unwrap_or(&0) as f64 / ccount as f64, 0.07);
	score += score_distance('p', *omap.get(&'p').unwrap_or(&0) as f64 / ccount as f64, 0.02);
	score += score_distance('q', *omap.get(&'q').unwrap_or(&0) as f64 / ccount as f64, 0.005);
	score += score_distance('r', *omap.get(&'r').unwrap_or(&0) as f64 / ccount as f64, 0.06);
	score += score_distance('s', *omap.get(&'s').unwrap_or(&0) as f64 / ccount as f64, 0.06);
	score += score_distance('t', *omap.get(&'t').unwrap_or(&0) as f64 / ccount as f64, 0.09);
	score += score_distance('u', *omap.get(&'u').unwrap_or(&0) as f64 / ccount as f64, 0.03);
	score += score_distance('v', *omap.get(&'v').unwrap_or(&0) as f64 / ccount as f64, 0.01);
	score += score_distance('w', *omap.get(&'w').unwrap_or(&0) as f64 / ccount as f64, 0.02);
	score += score_distance('x', *omap.get(&'x').unwrap_or(&0) as f64 / ccount as f64, 0.005);
	score += score_distance('y', *omap.get(&'y').unwrap_or(&0) as f64 / ccount as f64, 0.02);
	score += score_distance('z', *omap.get(&'z').unwrap_or(&0) as f64 / ccount as f64, 0.0);

	for word in target.split_whitespace() {
		if word == "the" || word == "I" || word == "some" || word == "You" || word == "you" || word == "can" {
			score += 100.0;
		}
	}

	/*for c in target.chars() {
		if c == ':' || c == '#' || c == '{' || c == '}' {
			return -10000.0
		}
	}

	let search: Vec<&str> = target.split("ss").collect();

	score += search.len() as f64 * 100.0;

	let search: Vec<&str> = target.split("the").collect();

	score += search.len() as f64 * 1000.0;*/

	score as f64
}