use colored::Colorize;
use dotenv::dotenv;
use memoria::{ui, MemorySize, Resource, Vault, VaultError};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Read environment variables with defaults
    let vault_name = env::var("VAULT_NAME").unwrap_or_else(|_| "Global Vault".to_string());
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

        // We wrap the logic in a closure that returns a Result<bool>
        // the bool indicates if we should continue the loop
        let result: Result<bool, Box<dyn Error>> = (|| {
            let command = ui::prompt(&available_commands)?.to_lowercase();

            match command.as_str() {
                "add" => {
                    handle_add(&mut my_vault)?;
                    Ok(true)
                }
                "summary" => {
                    my_vault.summary();
                    Ok(true)
                }
                "get" => {
                    handle_get(&my_vault)?;
                    Ok(true)
                }
                "delete" => {
                    handle_delete(&mut my_vault)?;
                    Ok(true)
                }
                "exit" => Ok(false),
                _ => Err(VaultError::InvalidInput("Invalid command".to_string()).into()),
            }
        })();

        match result {
            Ok(true) => continue,
            Ok(false) => break,
            Err(e) => eprintln!("{} {}", "Error:".red().bold(), e),
        }
    }

    Ok(())
}

fn handle_add(vault: &mut Vault<String>) -> Result<(), Box<dyn Error>> {
    let res_type = ui::prompt("What type? [text] [sensor] [log]\n> ")?;

    let resource = match res_type.to_lowercase().as_str() {
        "text" => {
            let text = ui::prompt("Enter text:\n> ")?;
            Resource::TextMessage(text)
        }
        "sensor" => {
            let val_str = ui::prompt("Enter value:\n> ")?;
            let val = val_str
                .parse::<f64>()
                .map_err(|_| VaultError::InvalidInput("Invalid number".to_string()))?;
            Resource::SensorData(val)
        }
        "log" => {
            let logs_str = ui::prompt("Enter logs (comma separated):\n> ")?;
            let logs = logs_str.split(',').map(|s| s.trim().to_string()).collect();
            Resource::SystemLogs(logs)
        }
        _ => return Err(VaultError::InvalidInput("Invalid type".to_string()).into()),
    };

    let key = ui::prompt("Enter a unique name (key) for this resource:\n> ")?;
    vault.add(key, resource)?;
    println!("Successfully added!");

    Ok(())
}

fn handle_get(vault: &Vault<String>) -> Result<(), Box<dyn Error>> {
    let key = ui::prompt("Enter name (key) of resource:\n> ")?;
    match vault.get(&key) {
        Some(resource) => println!("Resource: {:?}", resource),
        None => return Err(VaultError::ResourceNotFound(key).into()),
    }
    Ok(())
}

fn handle_delete(vault: &mut Vault<String>) -> Result<(), Box<dyn Error>> {
    let key = ui::prompt("Enter name (key) of resource to delete:\n> ")?;
    let res = vault.remove(&key)?;
    println!("Resource deleted successfully! {:?}", res);
    Ok(())
}
