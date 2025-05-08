use viuer::{is_iterm_supported, terminal_size};

pub fn _display_image_in_terminal(path: &str) {
    if !is_iterm_supported() {
        panic!("Display is not supported not in iterm");
    }
    let (w, h) = terminal_size();
    let conf = viuer::Config {
        width: Some(40),
        height: Some(30),
        x: w / 2,
        y: (h / 2) as i16,
        ..Default::default()
    };

    viuer::print_from_file(path, &conf).expect("Image printing failed.");
}
