use totp_rs::{Algorithm, Secret, TOTP};

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

    println!("{}", qr_code);
}
