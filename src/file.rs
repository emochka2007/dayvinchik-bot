use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

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
pub fn move_file(src: &str, dest: &str) -> std::io::Result<()> {
    fs::rename(src, dest).unwrap_or_else(|err| {
        // todo resolve why error is thrown even though the move is made
        // error!("Failed to move file: {}", err);
    });
    Ok(())
}