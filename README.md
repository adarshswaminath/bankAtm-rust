
# ATM CLI [Rust Learning]

A simple CLI-based Rust application simulating basic ATM functionality. This project is designed to help users deposit, withdraw, and check their account balance, all within the terminal.

## Features

- **User Verification**: Verifies the user using a secret PIN before granting access.
- **Deposit**: Allows users to deposit a specified amount into their balance.
- **Withdraw**: Enables users to withdraw funds, with checks for insufficient balance.
- **Balance Inquiry**: Displays the current account balance.
- **Exit**: Option to exit the application.

## Installation

1. **Clone the repository**:
    ```bash
    git clone https://github.com/adarshswaminathbankAtm-rust.git
    ```

2. **Navigate to the project directory**:
    ```bash
    cd bankAtm-rust
    ```

3. **Build the project** using Cargo:
    ```bash
    cargo build
    ```

4. **Run the project**:
    ```bash
    cargo run
    ```

## Dependencies

- **terminal_banner**: For displaying a banner in the terminal.
  
   This can be added to the \`Cargo.toml\` file like so:
   ```toml
   [dependencies]
   terminal_banner = "0.4"
   ```

## Usage

Once the program starts, you will be asked to enter your **Secret PIN** (currently hardcoded as `1234`). After successful verification, the following options will be available:

1. **Deposit**: Enter an amount to add to your balance.
2. **Withdraw**: Enter an amount to withdraw from your balance (ensures sufficient balance).
3. **Balance Inquiry**: Displays your current balance.
4. **Exit**: Exits the application.

## Code Overview

- `main.rs`: Contains the main logic of the ATM tool, including user verification, deposit, withdraw, and balance inquiry functionalities.
- `user_input_amount()`: Prompts the user for an amount.
- `verify_user()`: Verifies the user by checking a secret PIN.
- `banner()`: Displays a welcome banner.
- `menu_list()`: Displays a list of available options.
- `clear_terminal()`: Clears the terminal screen for better readability.

## Contribution

Feel free to open issues or pull requests if you want to contribute to this project.

