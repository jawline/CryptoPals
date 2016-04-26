use openssl::crypto::symm::{encrypt, decrypt};
use std::string::{String, FromUtf8Error};
use openssl::crypto::symm::Type::{AES_128_ECB, AES_128_CBC};
use string_score::*;
use hamming;
use std::cmp::Ordering;

pub fn decrypt_message(data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
	let iv = vec![0; 16];
	decrypt(AES_128_ECB, key, &iv, data)
}

pub fn encrypt_message(data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
	let iv = vec![0; 16];
	encrypt(AES_128_ECB, key, &iv, data)
}

pub fn cbc_decrypt(data: &Vec<u8>, block_size: usize, key: &Vec<u8>) -> Vec<u8> {
	let d_data = decrypt_message(data, key);
	let mut result = Vec::new();
	let mut prev_block = vec![0; block_size];

	let iterations = (data.len() / block_size) + if data.len() % block_size > 0 { 1 } else { 0 };

	for i in 0..iterations {
		let mut block: Vec<u8> = d_data.iter().skip(i * block_size).take(block_size).map(|&x| x).collect();

		for j in 0..block.len() {
			block[j] ^= prev_block[j];
		}

		prev_block = data.iter().skip(i * block_size).take(block_size).map(|&x| x).collect();
		result.extend(block);
	}

	result
}

pub fn find_english(data: &Vec<Vec<u8>>, key: &Vec<u8>) -> String {
	let mut possibles: Vec<String> = data.iter().map(|item| String::from_utf8(decrypt_message(item, key))).filter(|x| x.is_ok()).map(|x| x.unwrap()).collect();
	possibles.sort_by(|x, y| score_string(x).partial_cmp(&score_string(y)).unwrap_or(Ordering::Equal));
	possibles.pop().unwrap()
}

pub fn score_item(data: &Vec<u8>) -> f64 {
	0.0
}

pub fn guess_ecb(data: &Vec<Vec<u8>>) -> usize {
	0
}