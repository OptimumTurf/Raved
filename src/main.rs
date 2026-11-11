use chrono::Local;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs;
use std::path::Path;

fn main() {
    // Print current directory for debugging
    let current_dir = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
    println!("Current directory: {:?}", current_dir);
    
    // List files in current directory
    match fs::read_dir(".") {
        Ok(entries) => {
            println!("Files in current directory:");
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  {:?}", entry.path());
                }
            }
        },
        Err(e) => println!("Error reading directory: {}", e),
    }

    // Generate random content
    let timestamp = Local::now();
    let random_number: u32 = rand::thread_rng().gen_range(1..=1000);
    let random_text = format!("Random update at {}\nRandom number: {}\n", timestamp, random_number);

    // Append to a file directly in the repository root
    let file_path = "random.txt";
    println!("Attempting to open/create file: {}", file_path);
    
    match OpenOptions::new().append(true).create(true).open(file_path) {
        Ok(mut file) => {
            match writeln!(file, "{}", random_text) {
                Ok(_) => println!("Successfully wrote to file"),
                Err(e) => println!("Failed to write to file: {}", e),
            }
        },
        Err(e) => println!("Failed to open file: {}", e),
    }
    
    // Verify file exists after writing
    if Path::new(file_path).exists() {
        println!("Verified file exists after writing");
        
        // Print file contents for debugging
        match fs::read_to_string(file_path) {
            Ok(contents) => println!("File contents:\n{}", contents),
            Err(e) => println!("Error reading file: {}", e),
        }
    } else {
        println!("File does not exist after attempted write!");
    }
    
    println!("Random update generation complete!");
}