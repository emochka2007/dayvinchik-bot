use std::io;
use image::Luma;
use qrcode::QrCode;

fn _generate_qr_code(link: &str) {
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

