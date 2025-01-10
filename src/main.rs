//use std::{any::Any, collections::HashMap};
//
//fn main() {
//    let mut map: HashMap<String, Box<dyn Any>> = HashMap::new();
//    map.insert(String::from("Cube"),Box::new("You have placed a cube!".to_string()));
//    if let Some(value) = map.get("Cube") {
//        if let Some(number) = value.downcast_ref::<String>() {
//            println!("Number: {}", number);
//        }
//    }
//}



//=========================//
// This is where the code  //
// will begin--------------//
//  ▼    ▼    ▼    ▼    ▼  //
//=========================//



//   ▼    ▼    ▼    ▼    ▼  ▼    ▼    ▼    ▼ 
use std::{self, env, io::Write, path::Path, process::{Child, Command, Stdio}, thread::sleep, time::Duration};
mod lua;
mod rust;
use chrono::prelude::*;
//This is where the libraries will be handled

fn main() {
    external_scripts();
    std::thread::sleep(Duration::from_secs(4));

    // This code creates a shell enviroment.
    loop {
        let datetime: DateTime<Utc> = Utc::now();
        print!("Star-Engine[{}] ",datetime.format("%H:%M:%S").to_string());
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let command = input.trim();
        
        match command {
            "exit" => break,
            _ if command.starts_with("echo ") => {
                let output = &command[5..];
                println!("{}", output);
            },
            _ if command.starts_with("about") => {
                println!("                Star-Engine is a very customizable game engine supporting lua,blueprints and rust.
                We strive to help people get better at programming.
                We will be providing example templates for other users to use and learn.")
            }
            _ if command.starts_with("lua") => {
                lua::start_lua().unwrap_or_else(|e| println!("Lua error: {}", e));
            }
            _ if command.starts_with("rust") => {
                rust::start_rust_repl();
            }
            _ if command.starts_with("obj") => {
                load_obj();
            }
            _ => println!("Command not found: {}", command),
        }
    }
}

//We will be initializing other scripts here.
mod splash;
mod OBJ_loader_with_OBJ_files;
use OBJ_loader_with_OBJ_files::obj_loader::load_obj;


//Bare with me i know there is a better way but i dont know that way.

fn external_scripts() {
    splash::splash();
}
