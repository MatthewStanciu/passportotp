use totp_rs::{Algorithm, Secret, TOTP};

use crate::{
    util::display_qr_code::display_qr_code_browser, util::display_qr_code::display_qr_code_image,
    View,
};

pub fn generate(view: View) {
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

    let result = match view {
        View::Image => display_qr_code_image(&qr_code),
        View::Browser => display_qr_code_browser(&qr_code),
    };

    if let Err(e) = result {
      eprintln!("Failed to display QR code: {}", e);
  }
}
