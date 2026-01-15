use memoria::{Vault, VaultError, MemorySize, Resource, ui};

fn main() {
    let mut my_vault = Vault::new("Global Vault".to_string(), MemorySize::GB(50));

    println!("Welcome to Memoria!");

    loop {
        let mut available_commands = String::from("\nAvailable Commands: [add]");
        
        if !my_vault.resources.is_empty() {
            available_commands.push_str(" [summary] [get] [delete]");
        }
        
        available_commands.push_str(" [exit]\n> ");

        let command = match ui::prompt(&available_commands) {
            Ok(cmd) => cmd.to_lowercase(),
            Err(e) => {
                println!("Error reading input: {}", e);
                continue;
            }
        };

        match command.as_str() {
            "add" => {
                let res_type = match ui::prompt("What type? [text] [sensor] [log]\n> ") {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                

                let resource_result = match res_type.to_lowercase().as_str() {
                    "text" => {
                        match ui::prompt("Enter text:\n> ") {
                            Ok(text) => Ok(Resource::TextMessage(text)),
                            Err(e) => Err(e.to_string()),
                        }
                    }
                    "sensor" => {
                        match ui::prompt("Enter value:\n> ") {
                            Ok(val_str) => if let Ok(val) = val_str.parse::<f64>() {
                                Ok(Resource::SensorData(val))
                            } else {
                                Err("Invalid number".to_string())
                            },
                            Err(e) => Err(e.to_string()),
                        }
                    }
                    "log" => {
                        match ui::prompt("Enter logs (comma separated):\n> ") {
                            Ok(logs_str) => {
                                let logs = logs_str.split(',').map(|s| s.trim().to_string()).collect();
                                Ok(Resource::SystemLogs(logs))
                            }
                            Err(e) => Err(e.to_string()),
                        }
                    }
                    _ => Err("Invalid type".to_string()),
                };

                match resource_result {
                    Ok(resource) => {
                        // Ask for key only if resource creation was successful
                        match ui::prompt("Enter a unique name (key) for this resource:\n> ") {
                            Ok(key) => {
                                 match my_vault.add(key, resource) {
                                    Ok(_) => println!("Successfully added!"),
                                    Err(VaultError::VaultFull{..}) => println!("CRITICAL: Vault is full!"),
                                    Err(e) => println!("Error: {}", e), 
                                }
                            }
                            Err(e) => println!("Error reading key: {}", e),
                        }
                    }
                    Err(e) => println!("Error creating resource: {}", e),
                }
            }
            "summary" => {
                my_vault.summary();
            }
            "get" => {
                if let Ok(key) = ui::prompt("Enter name (key) of resource:\n>") {
                     match my_vault.get(&key) {
                        Some(resource) => println!("Resource: {:?}", resource),
                        None => println!("Error: No resource found with name '{}'", key),
                    }
                }
            }
            "delete" => {
                if let Ok(key) = ui::prompt("Enter name (key) of resource to delete:\n>") {
                    match my_vault.remove(&key) {
                        Ok(res) => println!("Resource deleted successfully! {:?}", res),
                        Err(e) => println!("Error deleting resource: {}", e),
                    }
                }
            }
            "exit" => {
                break;
            }
            _ => println!("Invalid command"),
        }
    }
}