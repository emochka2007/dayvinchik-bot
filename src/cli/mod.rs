use clap::Parser;
// use console::Emoji;
// https://lib.rs/crates/dialoguer
//https://docs.rs/indicatif/latest/indicatif/
//https://medium.com/better-programming/building-cli-apps-in-rust-what-you-should-consider-99cdcc67710c
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

pub fn parse_args() {
    loop {}
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}
