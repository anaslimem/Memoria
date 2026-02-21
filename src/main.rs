use colored::Colorize;
use dotenv::dotenv;
use memoria::{ui, MemorySize, Resource, Vault, VaultError};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Check for command line arguments
    let args: Vec<String> = env::args().collect();
    let should_persist = args.contains(&"--save".to_string());
    let vault_file = ".memoria/vault.json";

    // Read environment variables with defaults
    let vault_name = env::var("VAULT_NAME").unwrap_or_else(|_| "Global Vault".to_string());
    let capacity_gb: u64 = env::var("VAULT_CAPACITY_GB")
        .unwrap_or_else(|_| "50".to_string())
        .parse()
        .unwrap_or(50);

    // Try to load existing vault if persisting is enabled
    let mut my_vault = if should_persist && std::path::Path::new(vault_file).exists() {
        println!("Loading vault from {}...", vault_file);
        match Vault::<String>::load_from_file(vault_file) {
            Ok(vault) => {
                println!("✓ Vault loaded successfully!",);
                vault
            }
            Err(_) => {
                println!("Creating new vault...");
                Vault::<String>::new(vault_name.clone(), MemorySize::GB(capacity_gb))
            }
        }
    } else {
        // Create the vault with configurable name and capacity
        Vault::<String>::new(vault_name.clone(), MemorySize::GB(capacity_gb))
    };

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

    // Save vault on exit if persistence is enabled
    if should_persist {
        std::fs::create_dir_all(".memoria").ok();
        match my_vault.save_to_file(vault_file) {
            Ok(_) => println!("✓ Vault saved to {}", vault_file),
            Err(e) => eprintln!("⚠ Failed to save vault: {}", e),
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
        _ => {
            return Err(VaultError::InvalidInput("Invalid type".to_string()).into());
        }
    };

    let key = ui::prompt("Enter a unique name (key) for this resource:\n> ")?;
    vault.add(key, resource)?;
    println!("{}", "✓ Successfully added!".green().bold());

    Ok(())
}

fn handle_get(vault: &Vault<String>) -> Result<(), Box<dyn Error>> {
    let key = ui::prompt("Enter name (key) of resource:\n> ")?;
    match vault.get(&key) {
        Some(resource) => {
            println!("{}", "✓ Resource found!".green().bold());
            println!("Resource: {:?}", resource);
        }
        None => {
            return Err(VaultError::ResourceNotFound(key).into());
        }
    }
    Ok(())
}

fn handle_delete(vault: &mut Vault<String>) -> Result<(), Box<dyn Error>> {
    let key = ui::prompt("Enter name (key) of resource to delete:\n> ")?;
    let res = vault.remove(&key)?;
    println!("{}", "✓ Resource deleted successfully!".green().bold());
    println!("Deleted: {:?}", res);
    Ok(())
}
