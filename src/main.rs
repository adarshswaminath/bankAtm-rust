use terminal_banner::Banner;

fn main() {
    banner("Core Bank Of Rust");
    let option = menu_list();
    println!("Option Selected: {}", option);
}

fn banner(title: &str) {
    clear_terminal();
    let banner = Banner::new()
        .text(title.into())
        .text("Welcome to the Core Bank Of Rust".into())
        .render();
    println!("{}", banner)
}


fn menu_list() -> u32 {
    let mut user_option = String::new();
    println!("<============ Menu List ===========>");
    println!("1. Create Account");
    println!("2. Deposit");
    println!("3. Withdraw");
    println!("4. Balance Inquiry");
    println!("5. Exit");
    println!("Please Select a Option: ");
    std::io::stdin().read_line(&mut user_option).expect("Err -> Cannot read the input");
    let user_option: u32 = user_option.trim().parse().expect("Err -> Not a number");
    return user_option;
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
