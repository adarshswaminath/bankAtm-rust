use terminal_banner::Banner;

fn main() {
    let is_verified = verify_user(); // verification status
    let mut balance = 1000; // user account balance amount
    if is_verified {
        banner("Core Bank Of Rust");
        let option = menu_list();
        
        if option == 1 {
            // ? deposit amount
            let amount_deposite = user_input_amount();
            balance = balance + amount_deposite;
            println!("Your Balance ${}", balance);

        } else if option == 2 {
            // ? withdraw amount
            let amount_withdraw = user_input_amount();
            // ! Insufficient Balance
            if amount_withdraw > balance {
                println!("Insufficient Balance");
            }
            // ? withdraw amount and update balance
            balance = balance - amount_withdraw;
            println!("Your Balance ${}", balance);
        } else if option == 3 {
            println!("Your Balance ${}", balance);
        } else {
            println!("Thank you for using Core Bank Of Rust");
        }
    } else {
        println!("Access Denied");
    }
}

fn user_input_amount() -> u32 {
    let mut user_input = String::new();
    println!("Please Enter Amount: ");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Err -> Cannot read this line ");
    let user_input: u32 = user_input
        .trim()
        .parse()
        .expect("Err -> Error in convertion");
    return user_input;
}
/*
* @description: Verifies the user
* @return: boolean result of the verification
*/
fn verify_user() -> bool {
    clear_terminal();
    let password = 1234;
    let mut user_password_input = String::new();
    println!("Please Enter Your Secret PIN: ");
    std::io::stdin()
        .read_line(&mut user_password_input)
        .expect("Err -> Cannot read the input");
    let user_password_input: u32 = user_password_input
        .trim()
        .parse()
        .expect("Err -> Not a number");
    return user_password_input == password;
}

/*
* @description: Prints the banner
* @param title: The title of the banner
*/
fn banner(title: &str) {
    clear_terminal();
    let banner = Banner::new()
        .text(title.into())
        .text("Welcome to the Core Bank Of Rust".into())
        .render();
    println!("{}", banner)
}

/*
* @description: Prints the menu list
* @return: The selected option integer
*/
fn menu_list() -> u32 {
    let mut user_option = String::new();
    println!("<============ Menu List ===========>");
    println!("1. Deposit");
    println!("2. Withdraw");
    println!("3. Balance Inquiry");
    println!("4. Exit");
    println!("Please Select a Option: ");
    std::io::stdin()
        .read_line(&mut user_option)
        .expect("Err -> Cannot read the input");
    let user_option: u32 = user_option.trim().parse().expect("Err -> Not a number");
    return user_option;
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
