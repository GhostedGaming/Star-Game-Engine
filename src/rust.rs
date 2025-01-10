use std::io::Write;
use std::collections::{HashMap, HashSet};

use chrono::prelude::*;

pub fn start_rust_repl() {
    let mut variables: HashMap<String, String> = HashMap::new();
    
    println!("\x1b[94mRust Terminal Started - Type 'exit' to return to main shell\x1b[0m");
    
    loop {
        let datetime: DateTime<Local> = Local::now();
        print!("\x1b[38;2;255;165;0mrust[{}]>\x1b[0m ", datetime.format("%H:%M:%S").to_string());
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let command = input.trim();
        
        match command {
            "exit" => break,
            _ if command.starts_with("print!(") => {
                let content = command.trim_start_matches("print!(\"").trim_end_matches("\")");
                print!("{}", content);
                println!("");
                std::io::stdout().flush().unwrap();
            },
            _ if command.starts_with("println!(") => {
                let content = command.trim_start_matches("println!(\"").trim_end_matches("\")");
                println!("{}", content);
            },
            _ if command.starts_with("let") => {
                let parts: Vec<&str> = command.split('=').collect();
                if parts.len() == 2 {
                    let var_name = parts[0].trim().trim_start_matches("let ").trim_end();
                    let value = parts[1].trim();
                    variables.insert(var_name.to_string(), value.to_string());
                    println!("\x1b[32mVariable {} set to {}\x1b[0m", var_name, value);
                }
            },
            _ if command.starts_with("vec!") => {
                let content = command.trim_start_matches("vec![").trim_end_matches("]");
                let items: Vec<&str> = content.split(',').collect();
                println!("\x1b[32mVector created with {} items\x1b[0m", items.len());
            },
            _ if command.starts_with("fn") => {
                if command.contains("()") && command.contains("{") {
                    println!("\x1b[32mFunction defined\x1b[0m");
                }
            },
            _ if command.starts_with("warn") => {
                if command.contains("()") {
                    let content = command.trim_start_matches("println!(\"").trim_end_matches("\")");
                    println!("{}", content);
                }
            },
            _ if command == "help" => {
                println!("\x1b[36mAvailable commands:\x1b[0m");
                println!("  print!(\"text\")    - Print without newline");
                println!("  println!(\"text\")  - Print with newline");
                println!("  let x = value     - Define variable");
                println!("  vec![1,2,3]       - Create vector");
                println!("  fn name() {{}}    - Define function");
                println!("  exit             - Exit REPL");
            },
            
            _ => {
                let highlighted_command = command
                    .replace("fn ", "\x1b[38;2;255;20;147mfn\x1b[0m ")
                    .replace("let ", "\x1b[38;2;57;255;20mlet\x1b[0m ")
                    .replace("mut ", "\x1b[38;2;0;255;255mmut\x1b[0m ")
                    // Include all your syntax highlighting replacements here
                    ;
                println!("\x1b[31mError: Unknown or invalid command: {}\x1b[0m", highlighted_command);
            }
        }
    }
}
