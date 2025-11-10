use std::fs;

use base64::{Engine, engine::general_purpose};
use openssl::string;
use crate::{cryptography::Crypto, oci_storage_client::OciClient, utils::get_oci_pem_path};

mod cryptography;
mod utils;
mod oci_storage_client;


#[tokio::main]
async fn main() {
    let region: String = String::from("<region>");
    let namespace: String = String::from("<namespace>");
    let profile_name: String = String::from(""); // "" for [DEFAULT]
        

    let client: OciClient = OciClient::new(region, namespace, profile_name);

    let result: Vec<u8> = client.get_object("<bucket>", "<object>").await.unwrap();

     fs::write("./data", result).expect("Failed to write");
}
