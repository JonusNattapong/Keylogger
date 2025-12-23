use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::Rng;
use crate::config::encryption_key;

pub fn encrypt(data: &[u8]) -> Vec<u8> {
    let key_bytes = encryption_key();
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = rand::thread_rng().gen::<[u8; 12]>();
    let nonce_struct = Nonce::from_slice(&nonce);
    let ciphertext = cipher.encrypt(nonce_struct, data).unwrap();
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);
    result
}