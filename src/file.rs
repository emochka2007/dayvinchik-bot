use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::io::Write;

type Result = std::io::Result<()>;
// type FileLog<'a> = Cow<'a, str>;

pub fn file_log(data: String){
    let mut write_context = File::create("teleterm.json").unwrap();
    write_context
        .write_all(data.as_bytes())
        .unwrap();
}
pub fn log_append(data: String, path: &str) -> Result {
    let data = format!("{data}\n");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true) // Enable append mode
        .open(path)?;

    file.write_all(data.as_bytes())?;
    Ok(())
}
