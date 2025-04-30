use crate::auth::qr_auth_init;
use anyhow::Result;
use image::Luma;
use qrcode::QrCode;
use serde::Deserialize;
use std::io;

#[derive(Debug, Deserialize)]
pub struct UpdateAuthorizationState {
    authorization_state: AuthorizationState,
}

#[derive(Debug, Deserialize)]
struct AuthorizationState {
    link: String,
}
pub fn auth_tdlib(json_str: &str) -> Result<()> {
    let auth_state: UpdateAuthorizationState = serde_json::from_str(json_str)?;
    generate_qr_code(&auth_state.authorization_state.link);
    Ok(())
}
pub fn generate_qr_code(link: &str) {
    let code = QrCode::new(link.as_bytes()).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save("qr_code.png").unwrap();
    println!("✅ QR Code saved as 'qr_code.png'. Scan it with your Telegram app.");
}

pub fn input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}
