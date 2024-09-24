use terminal_banner::Banner;

fn main() {
    banner("Core Bank Of Rust");
}

fn banner(title: &str) {
    clear_terminal();
    let banner = Banner::new()
        .text(title.into())
        .text("Welcome to the Core Bank Of Rust".into())
        .render();
    println!("{}", banner)
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
