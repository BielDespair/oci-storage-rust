use std::env;
use std::path::PathBuf;
use std::collections::HashMap;

use ini::ini;

pub fn home_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        PathBuf::from(env::var("USERPROFILE").unwrap())
    } else {
        PathBuf::from(env::var("HOME").unwrap())
    }
}

pub fn get_oci_path() -> PathBuf {
    return home_dir()
        .join(".oci")
}
pub fn get_oci_pem_path() -> String {
    return get_oci_path()
        .join("oci_api_key.pem")
        .into_os_string()
        .into_string()
        .expect("Failed to get .pem string");
}

pub fn get_oci_config_path() -> String {
    return get_oci_path()
        .join("config")
        .into_os_string()
        .into_string()
        .expect("Failed to get OCI config file");
}


pub fn extract_key_id(path: String, name: String) -> String {
    let mut profile_name: &str = "default";

    if !name.is_empty() {
        profile_name = &name.as_str();
    }

    let conf:HashMap<String, HashMap<String, Option<String>>>= ini!(path.as_str());

    println!("{:?}", conf);
    let user: String = conf[profile_name]["user"]
        .clone()
        .unwrap_or_else(|| panic!("Could not find value 'user' on profile '{}'", profile_name));

    let fingerprint: String = conf[profile_name]["fingerprint"]
        .clone()
        .unwrap_or_else(|| panic!("Could not find value 'user' on profile '{}'", profile_name));

    let tenancy: String = conf[profile_name]["tenancy"]
        .clone()
        .unwrap_or_else(|| panic!("Could not find value 'user' on profile '{}'", profile_name));


    return format!("{}/{}/{}", tenancy, user, fingerprint);
}
