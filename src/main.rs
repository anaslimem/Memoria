use dotenv::dotenv;
use memoria::{error, ui, MemorySize, Resource, Vault, VaultError};
use std::env;

fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Read environment variables with defaults
    let vault_name = env::var("VAULT_NAME").unwrap_or_else(|_| "Default Vault".to_string());
    let capacity_gb: u64 = env::var("VAULT_CAPACITY_GB")
        .unwrap_or_else(|_| "50".to_string())
        .parse()
        .unwrap_or(50);

    // Create the vault with configurable name and capacity
    let mut my_vault = Vault::<String>::new(vault_name.clone(), MemorySize::GB(capacity_gb));

    println!("Welcome to Memoria - {}!", vault_name);

    loop {
        let mut available_commands = String::from("\nAvailable Commands: [add]");

        if !my_vault.resources.is_empty() {
            available_commands.push_str(" [summary] [get] [delete]");
        }

        available_commands.push_str(" [exit]\n> ");

        let command = match ui::prompt(&available_commands) {
            Ok(cmd) => cmd.to_lowercase(),
            Err(e) => {
                error::print_error(&VaultError::InvalidInput(format!(
                    "Error reading input: {}",
                    e
                )));
                continue;
            }
        };

        match command.as_str() {
            "add" => handle_add(&mut my_vault),
            "summary" => my_vault.summary(),
            "get" => handle_get(&my_vault),
            "delete" => handle_delete(&mut my_vault),
            "exit" => break,
            _ => error::print_error(&VaultError::InvalidInput("Invalid command".to_string())),
        }
    }
}

fn handle_add(vault: &mut Vault<String>) {
    let res_type = match ui::prompt("What type? [text] [sensor] [log]\n> ") {
        Ok(t) => t,
        Err(_) => return,
    };

    let resource_result = match res_type.to_lowercase().as_str() {
        "text" => match ui::prompt("Enter text:\n> ") {
            Ok(text) => Ok(Resource::TextMessage(text)),
            Err(e) => Err(format!("Error reading text: {}", e)),
        },
        "sensor" => match ui::prompt("Enter value:\n> ") {
            Ok(val_str) => match val_str.parse::<f64>() {
                Ok(val) => Ok(Resource::SensorData(val)),
                Err(_) => Err("Invalid number".to_string()),
            },
            Err(e) => Err(format!("Error reading value: {}", e)),
        },
        "log" => match ui::prompt("Enter logs (comma separated):\n> ") {
            Ok(logs_str) => {
                let logs = logs_str.split(',').map(|s| s.trim().to_string()).collect();
                Ok(Resource::SystemLogs(logs))
            }
            Err(e) => Err(format!("Error reading logs: {}", e)),
        },
        _ => Err("Invalid type".to_string()),
    };

    match resource_result {
        Ok(resource) => match ui::prompt("Enter a unique name (key) for this resource:\n> ") {
            Ok(key) => match vault.add(key, resource) {
                Ok(_) => println!("Successfully added!"),
                Err(e) => error::print_error(&e),
            },
            Err(e) => error::print_error(&VaultError::InvalidInput(format!(
                "Error reading key: {}",
                e
            ))),
        },
        Err(e) => error::print_error(&VaultError::InvalidInput(format!(
            "Error creating resource: {}",
            e
        ))),
    }
}

fn handle_get(vault: &Vault<String>) {
    if let Ok(key) = ui::prompt("Enter name (key) of resource:\n> ") {
        match vault.get(&key) {
            Some(resource) => println!("Resource: {:?}", resource),
            None => error::print_error(&VaultError::ResourceNotFound(key)),
        }
    }
}

fn handle_delete(vault: &mut Vault<String>) {
    if let Ok(key) = ui::prompt("Enter name (key) of resource to delete:\n> ") {
        match vault.remove(&key) {
            Ok(res) => println!("Resource deleted successfully! {:?}", res),
            Err(e) => error::print_error(&e),
        }
    }
}
