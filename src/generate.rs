use totp_rs::{Algorithm, Secret, TOTP};

use crate::util::display_qr_code::display_qr_code;

pub fn generate() {
    let secret = Secret::default().to_bytes().unwrap();
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret,
        Some("Purdue Hackers".to_string()),
        "12".to_string(), // TODO: get from login
    )
    .unwrap();

    let qr_code = match totp.get_qr_base64() {
        Ok(code) => code,
        Err(_e) => {
            return;
        }
    };

    // println!("{}", qr_code);
    match display_qr_code(&qr_code) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Failed to display QR code: {}", e);
        }
    }
}
