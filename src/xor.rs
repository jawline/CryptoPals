pub fn xor(origin: Vec<u8>, other: Vec<u8>) -> Vec<u8> {
	origin.iter().zip(other.iter()).map(|(x,y)| x ^ y).collect()
}