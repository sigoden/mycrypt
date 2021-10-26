use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Nonce};
use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use sha2::{Digest, Sha256};
use typenum::U12;

pub fn encrypt(pass: &str, plain_text: &[u8]) -> Result<Vec<u8>> {
    let nonce = get_nonce();
    let cipher = cipher_from_pass(pass);
    cipher
        .encrypt(&nonce, plain_text)
        .map_err(|e| anyhow!("Failed to encrypt, {}", e))
}

pub fn decrypt(pass: &str, cipher_text: &[u8]) -> Result<Vec<u8>> {
    let nonce = get_nonce();
    let cipher = cipher_from_pass(pass);
    cipher
        .decrypt(&nonce, cipher_text)
        .map_err(|_| anyhow!("Failed to decrypt, maybe incorrect password?"))
}

pub fn read_file(file: &PathBuf) -> Result<Vec<u8>> {
    let mut f = File::open(&file).map_err(|_| anyhow!("Failed to open file"))?;
    let mut b = Vec::<u8>::new();
    f.read_to_end(&mut b)
        .map_err(|_| anyhow!("Failed to read file"))?;
    Ok(b)
}

pub fn write_file(file: &PathBuf, data: &[u8]) -> Result<()> {
    let mut f = File::create(&file).map_err(|_| anyhow!("Failed to create file"))?;
    f.write_all(data)
        .map_err(|_| anyhow!("Failed to write file"))?;
    Ok(())
}

fn get_nonce() -> Nonce<U12> {
    Nonce::from_slice(b"xxxxxx nonce").to_owned()
}

fn cipher_from_pass(pass: &str) -> Aes256Gcm {
    let mut hasher = Sha256::new();
    hasher.update(pass);
    let key = hasher.finalize();
    Aes256Gcm::new(&key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let pass = "123456";
        let text = b"plaintext message";
        let cipher_text = encrypt(pass, text).unwrap();
        let plain_text = decrypt(pass, &cipher_text).unwrap();
        assert_eq!(&plain_text, text);
    }
}
