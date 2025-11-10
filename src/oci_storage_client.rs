use std::fmt::format;
use std::time::Duration;

use chrono::{Utc, format};
use openssl::sign;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, header};

use crate::cryptography::Crypto;
use crate::utils::{self, extract_key_id};

pub struct OciClient {
    http_client: Client,
    region: String,
    namespace: String,
    signer: Crypto,

    base_uri: String,
    host: String,
    key_id: String,
}

impl OciClient {
    pub fn new(region: String, namespace: String, profile_name: String) -> Self {
        let key_id: String = extract_key_id(utils::get_oci_config_path(), profile_name);
        let pem_path: String = utils::get_oci_pem_path();
        let signer: Crypto = Crypto::new(pem_path.as_str());

        let base_uri: String =
            format!("https://objectstorage.{region}.oraclecloud.com/n/{namespace}");
        let host: String = format!("objectstorage.{}.oraclecloud.com", region);

        let http_client: Client = Client::new();

        return Self {
            http_client,
            region,
            namespace,
            signer,

            base_uri,
            host,
            key_id,
        };
    }

    pub async fn get_object(&self, bucket: &str, object_name: &str) -> anyhow::Result<Vec<u8>> {
        let url: String = format!("{}/b/{}/o/{}", self.base_uri, bucket, object_name);

        println!("URL: {}", url);

        let path: String = format!("/n/{}/b/{}/o/{}", self.namespace, bucket, object_name);
        let headers: HeaderMap = self.build_oci_headers("get", &path);

        let res = self.http_client.get(url).headers(headers).send().await?;

        if res.status().is_success() {
            Ok(res.bytes().await?.to_vec())
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(anyhow::anyhow!("Request failed {}: {}", status, body))
        }
    }

    fn build_oci_headers(&self, method: &str, path: &str) -> HeaderMap {
        let mut headers: HeaderMap = HeaderMap::new();
        let date: String = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let authorization: String = self.build_authorization_header(&method, &path, date.clone());

        headers.insert("Date", HeaderValue::from_str(&date).unwrap());
        headers.insert("Host", HeaderValue::from_str(&self.host).unwrap());
        headers.insert("Authorization", HeaderValue::from_str(&authorization).unwrap(),
        );

        return headers;
    }

    fn build_authorization_header(&self, method: &str, path: &str, date: String) -> String {
        let signing_string: String = format!(
            "(request-target): {} {}\nhost: {}\ndate: {}",
            method.to_lowercase(),
            path,
            self.host,
            date
        );

        let signature_b64 = self.signer.sign_base64(&signing_string);
        
        println!("Date: {}", date);
        let header: String = format!(
        r#"Signature version="1",keyId="{}",algorithm="rsa-sha256",headers="(request-target) host date",signature="{}""#,
        self.key_id,
        signature_b64
        );

        println!("Authorization: {}", header);
        return header
    }

    /*
    pub fn generate_auth_header(&self, method: &str, path: &str) -> (String, String, String) {
        let date: String = Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string();
        let host = format!("objectstorage.{}.oraclecloud.com", self.region);

        return (date, date, date);
    }
     */
}
