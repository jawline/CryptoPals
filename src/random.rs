use rand::os::OsRng;
use rand::Rng;

pub fn generate_key(length: usize) -> Vec<u8> {
	let mut rng = OsRng::new().unwrap();
	(0..length).map(|x| rng.gen::<u8>()).collect()
}