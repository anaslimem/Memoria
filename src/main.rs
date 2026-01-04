use memoria::{Vault, MemorySize, Resource};
use std::io::{self, Write};
fn main() {
    let mut my_vault = Vault::new("Global Vault".to_string(), MemorySize::GB(50));

    println!("Welcome to Memoria!");

    loop{
        println!("\nAvailable Commands: [add] [summary] [get] [exit]");
        print!(">");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let command = input.trim().to_lowercase();
        match command.as_str(){
            "add" => {
                println!("What type of resource do you want to add? [text] [sensor] [log]");
                print!(">");
                io::stdout().flush().unwrap();
                let mut resource_type = String::new();
                io::stdin().read_line(&mut resource_type).expect("Failed to read line");
                let resource_type = resource_type.trim().to_lowercase();
                match resource_type.as_str(){
                    "text" => {
                        println!("Enter text for your message:");
                        print!(">");
                        io::stdout().flush().unwrap();
                        let mut text = String::new();
                        io::stdin().read_line(&mut text).expect("Failed to read line");
                        let resource = Resource::TextMessage(text.trim().to_string());
                        my_vault.add(resource);
                    }
                    "sensor" => {
                        println!("Enter sensor value (e.g. 24.5):");
                        print!(">");
                        io::stdout().flush().unwrap();
                        let mut sensor_data = String::new();
                        io::stdin().read_line(&mut sensor_data).expect("Failed to read line");
                        // SAFE PARSING:
                        if let Ok(val) = sensor_data.trim().parse::<f64>() {
                            my_vault.add(Resource::SensorData(val));
                        } else {println!("Warning: Invalid number! Resource not added.");
                        }
                    }
                    "log" => {
                        println!("Enter logs separated by commas (e.g., Boot successful, Login detected, Error 404):");
                        print!("> ");
                        io::stdout().flush().unwrap();
                        let mut logs = String::new();
                        io::stdin().read_line(&mut logs).expect("Failed to read line");
                        let logs: Vec<String> = logs.split(',').map(|s| s.trim().to_string()).collect();
                        let resource = Resource::SystemLogs(logs);
                        my_vault.add(resource);
                    }
                    _ => println!("Invalid resource type"),
                }
            }
            "summary" => {
                my_vault.summary();
            }
            "get" => {
                println!("Enter index of resource:");
                print!(">");
                io::stdout().flush().unwrap();
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).expect("Failed to read line");
                // SAFE PARSING:
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    match my_vault.get(index) {
                        Some(resource) => println!("Resource: {:?}", resource),
                        None => println!("Warning: No resource found at index {}", index),
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
