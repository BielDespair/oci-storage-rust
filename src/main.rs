use base64::{Engine, engine::general_purpose};

use crate::{cryptography::Crypto, oci_storage_client::OciClient};

mod cryptography;
mod utils;
mod oci_storage_client;

fn main() {
    
    let client: OciClient = OciClient::new();
    let header: String = String::from("Hello, Crypto!");
    
    //let crypto: Crypto = Crypto::new(&pem_path);
    //let signed: Vec<u8> = crypto.sign(header);
    //let signature_str = general_purpose::STANDARD.encode(signed);
    //println!("{:?}", signature_str);
}
