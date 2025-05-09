use crate::common::StdResult;
use base64::engine::general_purpose::STANDARD;
use base64::prelude::BASE64_STANDARD;
use base64::write::EncoderWriter;
use base64::Engine;
use log::{debug, error};
use std::fs::{read_dir, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, fs, io};
use tokio::time::sleep;

pub fn get_project_root() -> io::Result<PathBuf> {
    let path = env::current_dir()?;
    let path_ancestors = path.as_path().ancestors();

    for p in path_ancestors {
        let has_cargo = read_dir(p)?.any(|p| p.unwrap().file_name() == *"Cargo.lock");
        if has_cargo {
            return Ok(PathBuf::from(p));
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Ran out of places to find Cargo.toml",
    ))
}
pub fn file_log(data: String) {
    let mut write_context = File::create("teleterm.json").unwrap();
    write_context.write_all(data.as_bytes()).unwrap();
}

pub fn log_append(data: String, path: &str) -> StdResult {
    let data = format!("{data}\n");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true) // Enable append mode
        .open(path)?;

    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn read_json_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
pub fn move_file(src: &str, dest: &str) -> StdResult {
    fs::rename(src, dest).unwrap_or_else(|_err| {
        debug!("Failed to move file: {}", _err);
    });
    Ok(())
}
pub async fn get_image_with_retries(path_to_img: &str) -> io::Result<String> {
    let base64_image = {
        //todo config
        let max_attempts = 3;
        let mut attempts = 0;

        loop {
            match image_to_base64(path_to_img) {
                Ok(img) => {
                    break img;
                }
                Err(_e) => {
                    // todo Retries should be removed, as now  we check beforehand
                    error!("Cannot get the image. retrying {attempts}");
                    attempts += 1;
                    if attempts >= max_attempts {
                        return Err(io::Error::new(
                            io::ErrorKind::TimedOut,
                            format!(
                                "Max attempts reached. Image file still not found at '{}'. \
                                     Marked as failed and returning.",
                                path_to_img
                            ),
                        ));
                    } else {
                        sleep(Duration::from_secs(2)).await;
                    }
                }
            }
        }
    };
    Ok(base64_image)
}

pub fn new_base64(path: &str) -> String {
    let mut buffer = Vec::new();
    let mut input = File::open(path).unwrap();
    {
        let mut encoder = EncoderWriter::new(&mut buffer, &STANDARD);
        io::copy(&mut input, &mut encoder).unwrap();
        encoder.finish().unwrap();
    }
    String::from_utf8(buffer).unwrap()
}

pub fn image_to_base64(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let encoded = BASE64_STANDARD.encode(buffer);
    Ok(encoded)
}

pub fn file_exists(path: &str) -> bool {
    let file_path = Path::new(path);

    if file_path.exists() {
        return true;
    }
    false
}
