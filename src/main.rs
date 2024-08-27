use std::process::Command;
use std::fs;
use std::env;
use std::path::Path;

fn main() {
    // Step 1: Handle the creation of the network.toml file
    let network_toml_content = r#"
[relaychain]
chain = "polkadot-local"

[[relaychain.nodes]]
name = "alice"
validator = true

[[relaychain.nodes]]
name = "bob"
validator = true
"#;

    let network_toml_path = Path::new("./network.toml");

    // Check if the network.toml file exists
    if !network_toml_path.exists() {
        // Create and write to the network.toml file
        fs::write(network_toml_path, network_toml_content).expect("Failed to create network.toml file");
        println!("Created default network.toml file.");
    }

    // Step 2: Handle command-line arguments and run the `pop` command
    let args: Vec<String> = env::args().collect();

    // If no additional arguments are provided, use the default command
    let (command, command_args) = if args.len() == 1 {
        ("pop", vec!["up", "parachain", "-f", "./network.toml"])
    } else {
        // Pass the provided arguments directly to the `pop` command
        let cmd_args: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();
        ("pop", cmd_args)
    };

    // Run the command
    let status = Command::new(command)
        .args(&command_args)
        .status()
        .expect("Failed to execute command");

    if status.success() {
        println!("Command executed successfully.");
    } else {
        eprintln!("Command failed to execute.");
    }

    // Run the setup script if requested
    if args.contains(&"setup".to_string()) {
        let setup_status = Command::new("cargo")
            .args(&["run", "--release", "--", "setup"])
            .status()
            .expect("Failed to run the setup script");

        if !setup_status.success() {
            eprintln!("Setup script failed.");
        }
    }
}