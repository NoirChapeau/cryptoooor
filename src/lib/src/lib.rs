use wasm_bindgen::prelude::*;
use typenum;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};

#[wasm_bindgen]
pub fn encrypt_file(content: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    let aes_key = Key::<Aes256Gcm>::from_slice(&key);
    let (ciphertext, nonce) = encrypt(content, &aes_key)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    // Concatenate ciphertext and nonce
    let nonce_vec = nonce.as_slice().to_vec();
    Ok([ciphertext, nonce_vec].concat())
}

#[wasm_bindgen]
pub fn decrypt_file(encrypted_data: Vec<u8>, key: Vec<u8>) -> Result<Vec<u8>, JsValue> {
    let aes_key = Key::<Aes256Gcm>::from_slice(&key);
    decrypt(encrypted_data, aes_key).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn generate_key() -> Result<Vec<u8>, JsValue> {
    let key = Aes256Gcm::generate_key(&mut OsRng);
    Ok(key.to_vec())
}

fn encrypt(content: Vec<u8>, key: &Key<Aes256Gcm>) -> Result<(Vec<u8>, Nonce<typenum::U12>), Box<dyn std::error::Error>> {
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, &*content.as_ref())
        .map_err(|e| format!("Encryption failed: {}", e))?;
    Ok((ciphertext, nonce))
}

fn decrypt(encrypted_data: Vec<u8>, key: &Key<Aes256Gcm>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    const NONCE_SIZE: usize = 12;
    
    if encrypted_data.len() < NONCE_SIZE {
        println!("Damn size fucked up");
        //return Err(Box::new(format!("Invalid encrypted data size: {}", encrypted_data.len())));
    }
    
    let (ciphertext, nonce_vec) = encrypted_data.split_at(encrypted_data.len() - NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_vec);
    let cipher = Aes256Gcm::new(key);

    let decrypted_content = cipher.decrypt(nonce, &*ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;

    Ok(decrypted_content)
}
