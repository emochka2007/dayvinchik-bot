use std::{fs, io};
use std::io::Write;
use clap::{arg, command};
use colored::Colorize;
use log::info;
use crate::autoresponder::{create_jsonl_file};

pub fn is_cli_mode() -> bool {
    let matches = command!().arg(arg!([cli])).get_matches();
    if let Some(_) = matches.get_one::<String>("cli") {
        return true;
    }
    false
}

pub fn start_cli() -> anyhow::Result<()> {
    loop {
        print!("{}",
               "\r --- Choose an option ---\n 1. Transform chats\n 2. ---\n".magenta()
        );

        print!("{}", "Enter your choice: ".yellow());
        io::stdout().flush()?;

        let mut menu_option_input = String::new();
        io::stdin()
            .read_line(&mut menu_option_input)
            .expect("Failed to read input");

        // Trim the input and match the options
        match menu_option_input.trim() {
            "1" => {
                let paths = fs::read_dir("./tg_private_chats")?;
                for file in paths {
                    let file_name = file?.path();
                    let file_name_str = file_name.to_str().unwrap();
                    if file_name_str.ends_with(".json") {
                        info!("Parsing {file_name_str}");
                        create_jsonl_file(file_name_str)?;
                    }
                }
                info!("Successfully parsed all files");
                return Ok(());
            }
            "2" => {}
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}