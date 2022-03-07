use md5::{Digest, Md5};
use std::env;
use std::fs;
use std::path::Path;
use x509_parser::prelude::*;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hash.rs");

    let cert_content = fs::read("certs/ca_cert.der").unwrap();
    let hash = calculate_subject_hash(&cert_content);

    fs::write(
        &dest_path,
        format!("const SUBJECT_HASH: &str = \"{}\";", hash),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=certs/ca_cert.der");
}

fn calculate_subject_hash(cert: &[u8]) -> String {
    let (_rem, x509) = X509Certificate::from_der(cert).unwrap();

    let mut hasher = Md5::new();
    hasher.update(x509.subject().as_raw());
    let hash = hasher.finalize();

    let mut h = hash[0..4].to_vec();
    h.reverse();

    hex::encode(&h)
}
