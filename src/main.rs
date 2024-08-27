use std::process::Command;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::env;
use std::path::Path;

fn main() {
    // Step 1: Add the alias
    let alias_command = "alias .='./polkadot'";

    // Determine which shell is being used and select the correct config file
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    let config_file = if shell.contains("zsh") {
        ".zshrc"
    } else {
        ".bashrc"
    };

    // Get the path to the home directory and config file
    let home_dir = env::var("HOME").expect("Failed to get home directory");
    let config_path = Path::new(&home_dir).join(config_file);

    // Open the shell configuration file in append mode
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&config_path)
        .expect("Failed to open shell configuration file");

    // Write the alias command to the file
    writeln!(file, "{}", alias_command).expect("Failed to write alias to file");

    println!("Alias added to {}", config_file);

    // Source the updated configuration file to apply the alias immediately
    let status = Command::new("sh")
        .arg("-c")
        .arg(format!("source {}", config_path.to_str().unwrap()))
        .status()
        .expect("Failed to source shell configuration file");

    if status.success() {
        println!("Alias applied successfully. You can now run your script with `.`.");
    } else {
        eprintln!("Failed to apply the alias immediately. Please restart your terminal.");
    }

    // Step 2: Run the command `pop up parachain -f ./network.toml`
    let command = "pop";
    let args = vec!["up", "parachain", "-f", "./network.toml"];

    let status = Command::new(command)
        .args(&args)
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("Command executed successfully.");
    } else {
        eprintln!("Command failed to execute.");
    }

    // Optional: Wait for user input before closing (if running in an environment that closes quickly)
    print!("Press Enter to exit...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}
