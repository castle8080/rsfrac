use rcgen::generate_simple_self_signed;

use std::fs::create_dir;
use std::panic;
use std::path::Path;

///
/// Recursively creates a directory path if it does not exist.
///
fn create_dir_path(path: &Path) -> Result<(), std::io::Error> {
    if !path.is_dir() {
        if let Some(parent_path) = path.parent() {
            create_dir_path(&parent_path)?;
            create_dir(path)?;
        }
    }
    Ok(())
}

///
/// Create the parent directory if it does not exist.
/// 
fn create_parent_dir_if_not_exists(path: &Path) -> Result<(), std::io::Error> {
    if let Some(parent_path) = path.parent() {
        create_dir_path(parent_path)?;
    }
    Ok(())
}

///
/// Creates a self signed certificate and key file
/// if the files do not exist.
/// 
pub fn generate_self_signed_certs_if_not_exists(
    cert_path: &str,
    key_path: &str,
) -> Result<(), std::io::Error> {
    let cert_p = Path::new(cert_path);
    let key_p = Path::new(key_path);

    if cert_p.is_file() && key_p.is_file() {
        return Ok(());
    } else if key_p.is_file() {
        panic!("A cert file does not exist.");
    } else if cert_p.is_file() {
        panic!("A key file does not exist.");
    }

    // TODO: remove unwrap and convert to errors.

    let subject_alternate_names = vec!["localhost".to_string()];
    let cert = generate_simple_self_signed(subject_alternate_names).unwrap();
    let cert_pem = cert.serialize_pem().unwrap();
    let cert_key = cert.serialize_private_key_pem();

    create_parent_dir_if_not_exists(&cert_p)?;
    create_parent_dir_if_not_exists(&key_p)?;

    std::fs::write(cert_p, cert_pem)?;
    std::fs::write(key_p, cert_key)?;

    Ok(())
}
