use std::io::{self, Write};
use std::fs::File;
use std::path::Path;

fn main() -> io::Result<()> {
    let subject_alt_names = vec!["goldcoders.dev".to_string(), "localhost".to_string()];
    let cert = rcgen::generate_simple_self_signed(subject_alt_names)
        .expect("Failed to generate certificate");

    let dir_path = Path::new("cert");
    if !dir_path.exists() {
        std::fs::create_dir(dir_path)?;
    }

    let privkey_path = dir_path.join("private_key.pem");
    let mut file = File::create(privkey_path)?;
    file.write_all(cert.serialize_private_key_pem().as_bytes())?;

    let cert_path = dir_path.join("cert.pem");
    let mut file = File::create(cert_path)?;

    match cert.serialize_pem() {
        Ok(pem) => file.write_all(pem.as_bytes())?,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    }

    Ok(())
}
