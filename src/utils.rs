use std::env;
use std::path::PathBuf;

pub fn home_dir() -> PathBuf {
    println!("home_dir");
    if cfg!(target_os = "windows") {
        println!("Windows OS");
        PathBuf::from(env::var("USERPROFILE").unwrap())
    } else {
        println!("Linux OS");
        PathBuf::from(env::var("HOME").unwrap())
    }
}

pub fn get_oci_pem_path() -> String {
    return home_dir()
        .join(".oci/oci_api_key.pem")
        .into_os_string()
        .into_string()
        .expect("Failed to get .pem string");
}
