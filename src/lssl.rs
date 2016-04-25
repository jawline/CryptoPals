use openssl::crypto::symm::{encrypt, decrypt};
use std::string::{String, FromUtf8Error};
use openssl::crypto::symm::Type::AES_128_ECB;
use string_score::*;
use hamming;
use std::cmp::Ordering;

pub fn decrypt_message(data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
	let iv = vec![0; 128];
	decrypt(AES_128_ECB, key, &iv, data)
}

pub fn encrypt_message(data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
	let iv = vec![0; 128];
	encrypt(AES_128_ECB, key, &iv, data)
}

pub fn cbc_decrypt(data: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
	encrypt_message(data, key)
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