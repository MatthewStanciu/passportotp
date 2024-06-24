use base64::{engine::general_purpose, Engine as _};
use image::load_from_memory;
use open;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use viuer::Config;

pub fn display_qr_code_image(base64_data: &str) -> Result<(), Box<dyn Error>> {
    let qr_code_data = general_purpose::STANDARD.decode(base64_data)?;
    let img = load_from_memory(&qr_code_data)?;

    let viuer_conf = Config {
        height: Some(35),
        ..Default::default()
    };
    viuer::print(&img, &viuer_conf)?;

    println!("\nNot scanning? Try running this command with flag `--view browser`");

    Ok(())
}

pub fn display_qr_code_browser(base64_data: &str) -> Result<(), Box<dyn Error>> {
    let html_content = format!(
        "<html><body><img src=\"data:image/png;base64,{}\" /></body></html>",
        base64_data
    );

    let mut file_path = env::temp_dir();
    file_path.push("qr_code.html");

    let mut file = File::create(&file_path)?;
    file.write_all(html_content.as_bytes())?;

    open::that(&file_path)?;

    Ok(())
}
