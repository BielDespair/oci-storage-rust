use std::fs;

use base64::Engine;
use base64::engine::general_purpose;
use openssl::pkey::Private;
use openssl::{hash::MessageDigest, pkey::PKey, rsa::Rsa, sign::Signer};

pub struct Crypto {
    key: PKey<Private>,
}

impl Crypto {
    pub fn new(pem_path: &str) -> Self {
        let pem: Vec<u8> = fs::read(pem_path).expect(format!("Failed to read PEM key file at '{}'", pem_path).as_str());
        let rsa: Rsa<Private> = Rsa::private_key_from_pem(pem.as_slice()).unwrap();
        let key: PKey<Private> = PKey::from_rsa(rsa).unwrap();

        return Self { key };
    }

    pub fn sign(&self, string: &str) -> Vec<u8> {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.key).unwrap();
        signer.update(string.as_bytes()).unwrap();
        return signer.sign_to_vec().expect("Failed to sign string");
    }

    pub fn sign_base64(&self, string: &str) -> String {
        let bytes: Vec<u8> = self.sign(string);
        general_purpose::STANDARD.encode(bytes)
    }
}
