use std::process::Command;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::env;
use std::path::Path;

fn main() {
    // Define the alias command and its associated line
    let alias_command = "alias .='polkadot'";

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

    // Function to check if the alias already exists in the config file
    fn alias_exists(file_path: &Path, alias_line: &str) -> bool {
        if let Ok(contents) = fs::read_to_string(file_path) {
            contents.contains(alias_line)
        } else {
            false
        }
    }

    // Check if the alias already exists
    if !alias_exists(&config_path, alias_command) {
        // Open the shell configuration file in append mode
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&config_path)
            .expect("Failed to open shell configuration file");

        // Write the alias command to the file
        writeln!(file, "{}", alias_command).expect("Failed to write alias to file");

        println!("Alias added to {}", config_file);

        // Attempt to apply the alias immediately
        let shell_cmd = if shell.contains("zsh") { "zsh" } else { "bash" };
        let status = Command::new(shell_cmd)
            .arg("-c")
            .arg(format!("source {}", config_path.to_str().unwrap()))
            .status()
            .expect("Failed to source shell configuration file");

        if status.success() {
            println!("Alias applied successfully. You can now run your script with `.`.");
        } else {
            eprintln!("Failed to apply the alias immediately. Please restart your terminal or run `source {}` manually.", config_path.display());
        }
    } else {
        println!("Alias already exists in {}.", config_file);
    }

    // Compile the binary
    let compile_status = Command::new("cargo")
        .args(&["build", "--release"])
        .status()
        .expect("Failed to compile the project");

    if !compile_status.success() {
        eprintln!("Compilation failed.");
        return;
    }

    // Path to the compiled binary
    let binary_path = Path::new("target/release/polkadot");

    // Copy the binary to /usr/local/bin
    let copy_status = Command::new("sudo")
        .args(&["cp", binary_path.to_str().unwrap(), "/usr/local/bin/"])
        .status()
        .expect("Failed to copy binary to /usr/local/bin");

    if !copy_status.success() {
        eprintln!("Failed to copy the binary to /usr/local/bin.");
        return;
    }

    // Set the correct permissions for the binary
    let chmod_status = Command::new("sudo")
        .args(&["chmod", "+x", "/usr/local/bin/polkadot"])
        .status()
        .expect("Failed to set permissions for the binary");

    if !chmod_status.success() {
        eprintln!("Failed to set permissions for the binary.");
    } else {
        println!("Binary copied to /usr/local/bin and permissions set.");
    }
}
