use std::fs;

use openssl::pkey::Private;
use openssl::{hash::MessageDigest, pkey::PKey, rsa::Rsa, sign::Signer};

pub struct Crypto {
    key: PKey<Private>,
}

impl Crypto {
    pub fn new(pem_path: &str) -> Self {
        let pem: Vec<u8> = fs::read(pem_path).expect(".pem key not found");
        let rsa: Rsa<Private> = Rsa::private_key_from_pem(pem.as_slice()).unwrap();
        let key: PKey<Private> = PKey::from_rsa(rsa).unwrap();

        return Self { key };
    }

    pub fn sign(&self, string: String) -> Vec<u8> {
        let mut signer = Signer::new(MessageDigest::sha256(), &self.key).unwrap();
        signer.update(string.as_bytes()).unwrap();
        return signer.sign_to_vec().expect("Failed to sign string");
    }
}
