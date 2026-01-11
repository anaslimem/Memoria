use memoria::{Vault, MemorySize, Resource, ui};

fn main() {
    let mut my_vault = Vault::new("Global Vault".to_string(), MemorySize::GB(50));

    println!("Welcome to Memoria!");

    loop {
        let mut available_commands = String::from("\nAvailable Commands: [add]");
        
        if !my_vault.resources.is_empty() {
            available_commands.push_str(" [summary] [get] [delete]");
        }
        
        available_commands.push_str(" [exit]\n> ");

        let command = ui::prompt(&available_commands).to_lowercase();
        match command.as_str(){
            "add" => {
                let res_type = ui::prompt("What type? [text] [sensor] [log]\n> ");
                
                let result = match res_type.to_lowercase().as_str() {
                    "text" => {
                        let text = ui::prompt("Enter text:\n> ");
                        my_vault.add(Resource::TextMessage(text))
                    }
                    "sensor" => {
                        let val_str = ui::prompt("Enter value:\n> ");
                        if let Ok(val) = val_str.parse::<f64>() {
                            my_vault.add(Resource::SensorData(val))
                        } else {
                            Err("Invalid number".to_string())
                        }
                    }
                    "log" => {
                        let logs_str = ui::prompt("Enter logs (comma separated):\n> ");
                        let logs = logs_str.split(',').map(|s| s.trim().to_string()).collect();
                        my_vault.add(Resource::SystemLogs(logs))
                    }
                    _ => Err("Invalid type".to_string()),
                };

                // Handle the Result here!
                match result {
                    Ok(_) => println!("Successfully added!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "summary" => {
                my_vault.summary();
            }
            "get" => {
                let index_str = ui::prompt("Enter index of resource:\n>");
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    match my_vault.get(index) {
                        Some(resource) => println!("Resource: {:?}", resource),
                        None => println!("Warning: No resource found at index {}", index),
                    }
                } else {println!("Warning: Please enter a valid whole number for the index.");}
            }
            "delete" => {
                let index_str = ui::prompt("Enter index of resource to delete:\n>");
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    match my_vault.remove(index) {
                        Ok(res) => println!("Resource deleted successfully!{:?}", res),
                        Err(e) => println!("Error deleting resource: {}", e),
                    }
                } else {println!("Warning: Please enter a valid whole number for the index.");}
            }
            "exit" => {
                break;
            }
            _ => println!("Invalid command"),
        }
    }
}