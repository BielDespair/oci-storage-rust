
use std::path::PathBuf;

use crate::cryptography::Crypto;
use crate::utils;
use chrono::Utc;

pub struct OciClient {
    signer: Crypto,
}


const REGION: &str = "<region>";
const NAMESPACE: &str = "<namespace>";
const BUCKET: &str = "<bucket>";
const OBJECT_KEY: &str = "<file>";

impl OciClient {

    pub fn new() -> Self {


    let key_id: String = format!("{}/{}/{}", USER, FINGERPRINT, TENANCY);
        let pem_path: String = utils::get_oci_pem_path();
        let signer: Crypto = Crypto::new(pem_path.as_str());

        let final_uri = format!("https://objectstorage.{REGION}.oraclecloud.com/n/{NAMESPACE}/b/{BUCKET}/o/{OBJECT_KEY}");

        println!("final uri should be: {final_uri}");

        return Self {
            signer,

        }
    }

    /*
    pub fn generate_auth_header(&self, method: &str, path: &str) -> (String, String, String) {
        let date: String = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let host = format!("objectstorage.{}.oraclecloud.com", self.region);

        return (date, date, date);
    }
     */
}