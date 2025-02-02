use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::Deserialize;

pub fn file_log(data: String){
    let mut write_context = File::create("teleterm.json").unwrap();
    write_context
        .write_all(data.as_bytes())
        .unwrap();
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

pub fn read_json_file (path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}