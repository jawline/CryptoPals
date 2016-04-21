use std::collections::HashMap;

pub fn score_string(target: &str) -> i64 {

	let mut omap = HashMap::<char, usize>::new();

	for c in target.chars() {
		let new = omap.get(&c).unwrap_or(&0) + 1;
		omap.insert(c, new);
	}

	let score = *omap.get(&'e').unwrap_or(&0) + *omap.get(&'i').unwrap_or(&0) + *omap.get(&'o').unwrap_or(&0) + *omap.get(&'u').unwrap_or(&0);

	score as i64
}