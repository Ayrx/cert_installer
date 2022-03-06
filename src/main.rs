use anyhow::Result;
use md5::{Digest, Md5};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use x509_parser::prelude::*;

fn main() -> Result<()> {
    let ca_cert_bytes = include_bytes!("../certs/ca_cert.der");
    let subject_hash = calculate_subject_hash(ca_cert_bytes)?;

    // Save all content of the cacerts directory.
    let mut certs = HashMap::new();
    let cert_paths = fs::read_dir("/system/etc/security/cacerts")?;
    for entry in cert_paths {
        let path = entry?.path();
        let data = fs::read(&path)?;
        certs.insert(path.file_name().unwrap().to_os_string(), data);
    }

    // Mount a tmpfs over the cacerts directory.
    Command::new("mount")
        .args(["-t", "tmpfs", "tmpfs", "/system/etc/security/cacerts"])
        .status()?;

    // Write all of the original contents back to the tmpfs.
    let base = Path::new(&"/system/etc/security/cacerts");
    for (name, data) in certs.into_iter() {
        let f = base.join(&name);
        fs::write(f, data)?;
    }

    // Write the new certificate to the cacerts directory.
    fs::write(base.join(format!("{}.0", subject_hash)), ca_cert_bytes)?;

    // Ensure that all contents of the cacerts directory have the appropriate
    // permissions.
    Command::new("/bin/sh")
        .arg("-c")
        .arg("chown root:root /system/etc/security/cacerts/*")
        .status()?;

    Command::new("/bin/sh")
        .arg("-c")
        .arg("chmod 644 /system/etc/security/cacerts/*")
        .status()?;

    Command::new("/bin/sh")
        .arg("-c")
        .arg("chcon u:object_r:system_file:s0 /system/etc/security/cacerts/*")
        .status()?;

    Ok(())
}

fn calculate_subject_hash(cert: &[u8]) -> Result<String> {
    let (_rem, x509) = X509Certificate::from_der(cert)?;

    let mut hasher = Md5::new();
    hasher.update(x509.subject().as_raw());
    let hash = hasher.finalize();

    let mut h = hash[0..4].to_vec();
    h.reverse();
    let subject_hash = hex::encode(&h);

    Ok(subject_hash)
}
