use std::env;

// NOTE: See https://stackoverflow.com/q/40602708
fn main() {
    let root_dir = env::current_dir().unwrap();

    println!("cargo:rustc-link-lib=dylib=tdjson");
    println!(
        "cargo:rustc-link-search=native={}/../td/tdlib/lib/",
        root_dir.display()
    );
    println!(
        "cargo:rustc-link-arg=-Wl,-rpath,{}/../td/tdlib/lib/",
        root_dir.display()
    );
}
