use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::ffi::OsString;
use std::fs::{read_dir, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{env, fs, io};
use std::time::Duration;
use log::error;
use tokio::time::sleep;

pub fn get_project_root() -> io::Result<PathBuf> {
    let path = env::current_dir()?;
    let mut path_ancestors = path.as_path().ancestors();

    while let Some(p) = path_ancestors.next() {
        let has_cargo = read_dir(p)?
            .into_iter()
            .any(|p| p.unwrap().file_name() == OsString::from("Cargo.lock"));
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

pub fn log_append(data: String, path: &str) -> std::io::Result<()> {
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
pub fn move_file(src: &str, dest: &str) -> io::Result<()> {
    fs::rename(src, dest).unwrap_or_else(|_err| {
        // todo resolve why error is thrown even though the move is made
        // error!("Failed to move file: {}", err);
    });
    Ok(())
}
pub async fn get_image_with_retries(path_to_img: &str) -> io::Result<String> {
    let base64_image = {
        //todo config
        let max_attempts = 3;
        let mut attempts = 0;

        loop {
            match image_to_base64(&path_to_img) {
                Ok(img) => {
                    break img;
                }
                Err(e) => {
                    attempts += 1;
                    error!(
                                "Failed to load image to base64 (attempt {}/{}): {}",
                                attempts, max_attempts, e
                            );

                    if attempts >= max_attempts {
                        error!(
                                    "Max attempts reached. Image file still not found at '{}'. \
                                     Marked as failed and returning.",
                                    path_to_img
                                );
                        return Err(io::Error::new(
                            io::ErrorKind::TimedOut,
                            "Ran out of places to find Cargo.toml",
                        ));
                    } else {
                        sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        }
    };
    Ok(base64_image)
}
pub fn image_to_base64(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let encoded = BASE64_STANDARD.encode(buffer);
    Ok(encoded)
}
