use base64::{engine::general_purpose, Engine as _};
use image::load_from_memory;
use std::error::Error;
use viuer::Config;

pub fn display_qr_code(base64_data: &str) -> Result<(), Box<dyn Error>> {
    let qr_code_data = general_purpose::STANDARD.decode(base64_data)?;
    let img = load_from_memory(&qr_code_data)?;

    let viuer_conf = Config {
        height: Some(35),
        ..Default::default()
    };
    viuer::print(&img, &viuer_conf)?;

    Ok(())
}
