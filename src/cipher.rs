use chacha20poly1305::{aead::{Aead, OsRng}, AeadCore as _, ChaCha20Poly1305, Key, KeyInit as _, Nonce};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

pub const DEFAULT_ITERATIONS: u32 = 100_000;
pub const DEFAULT_KEY_LENGTH: usize = 32;

/// Derive a cryptographic key from a master password
pub fn derive_key(password: &[u8], salt: &[u8], iterations: u32, key_length: usize) -> Vec<u8> {
    let mut key = vec![0u8; key_length];
    pbkdf2_hmac::<Sha256>(password, salt, iterations, &mut key);
    key
}

pub fn encrypt_data(plaintext: &[u8], key: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher.encrypt(&nonce, plaintext).unwrap();

    (ciphertext, nonce.to_vec())
}

pub fn decrypt_data(ciphertext: &[u8], nonce: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);

    cipher.decrypt(nonce, ciphertext).unwrap()
}
