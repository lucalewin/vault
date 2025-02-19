mod cipher;

use cipher::{decrypt_data, derive_key, encrypt_data, DEFAULT_ITERATIONS, DEFAULT_KEY_LENGTH};
use rand::RngCore;

fn random_array(len: usize) -> Vec<u8> {
    let mut array = vec![0u8; len];
    rand::rng().fill_bytes(&mut array);
    array
}

fn signup(password: &str) -> (String, Vec<u8>, Vec<u8>, Vec<u8>) {
    let recovery_phrase = random_array(32);
    let salt = random_array(16);

    // derive the master key from the password
    let master_key = derive_key(password.as_bytes(), &salt, DEFAULT_ITERATIONS, DEFAULT_KEY_LENGTH);
    // derive the recovery key from the recovery phrase
    let recovery_key = derive_key(&recovery_phrase, &salt, DEFAULT_ITERATIONS, DEFAULT_KEY_LENGTH);

    // encrypt the master key with the recovery key
    let (recovery_code, recovery_nonce) = encrypt_data(&master_key, &recovery_key);

    println!("recovery key: {}", hex::encode(&recovery_key));
    
    (hex::encode(recovery_phrase), recovery_code, recovery_nonce, salt)
}

fn use_recovery_code(recovery_phrase: &str, recovery_salt: &[u8], recovery_code: &[u8], recovery_nonce: &[u8]) -> Vec<u8> {
    let recovery_phrase = hex::decode(recovery_phrase).unwrap();
    let key = derive_key(&recovery_phrase, recovery_salt, DEFAULT_ITERATIONS, DEFAULT_KEY_LENGTH);
    println!("recovery key: {}", hex::encode(&key));
    
    // dbg!(&recovery_code);
    decrypt_data(recovery_code, recovery_nonce, &key)
}

fn main() {
    let password = "super_secure_password";
    let mut salt = [0u8; 16];
    rand::rng().fill_bytes(&mut salt);

    let recovery = signup(password);

    let key = derive_key(password.as_bytes(), &salt, 100_000, 32);
    println!("Derived key: {}", hex::encode(&key));
    
    let plaintext = b"Hello, World!";
    let (ciphertext, nonce) = encrypt_data(plaintext, &key);
    println!("{ciphertext:?}");
    println!("{nonce:?}");
    drop(key);
    let key = use_recovery_code(&recovery.0, &recovery.3, &recovery.1, &recovery.2);
    println!("Derived key: {}", hex::encode(&key));
    let plain = decrypt_data(&ciphertext, &nonce, &key);
    dbg!(String::from_utf8(plain).unwrap());
}
