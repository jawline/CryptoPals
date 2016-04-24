pub fn pad_to(data: &Vec<u8>, num: usize, with: u8) -> Vec<u8> {
	data.iter().map(|&x| x).chain((0..num).map(|_| with)).collect()
}

pub fn pad_block(data: &Vec<u8>, block_size: usize, with: u8) -> Vec<u8> {
	pad_to(data, ((data.len() % block_size) + block_size) - data.len(), with)
}