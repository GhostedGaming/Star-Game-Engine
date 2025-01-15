use std::{self, env, io::{self, Write}, path::Path, process::{Child, Command, Stdio}, thread::{self, sleep}, time::{self, Duration}};
pub mod lua;
mod splash;
mod obj_loader_with_obj_files;
mod egui_test;
mod sausage_3d;
use obj_loader_with_obj_files::obj_loader::load_obj;
mod rust;
use chrono::prelude::*;
use tokio;
use std::process;

//This is where the libraries will be handled

#[tokio::main]
async fn main() {
    // Run the terminal in a separate thread since it's not UI-dependent
    let terminal_thread = tokio::spawn(async {
        terminal().await
    });

    // Run egui and other UI components directly on the main thread
    external_scripts().await;
    
    // Wait for terminal thread to complete if needed
    let _ = terminal_thread.await;
}

//We will be initializing other scripts here.

//Bare with me i know there is a better way but i dont know that way.

async fn external_scripts(){
    splash::splash();
    egui_test::src::main::main().expect("");
}

async fn terminal() {
    println!("This is the star engine console type 'about' to show the about section or type 'help' to show a list of commands");

    // This code creates a shell enviroment.
    loop {
        let datetime: DateTime<Utc> = Utc::now();
        print!("Star-Engine[{}] ",datetime.format("%H:%M:%S").to_string());
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let command = input.trim();
        
        match command {
            "exit" => process::exit(0),
            _ if command.starts_with("echo ") => {
                let output = &command[5..];
                println!("{}", output);
            },
            _ if command.starts_with("about") => {
                println!("Star-Engine is a very customizable game engine supporting lua,blueprints and rust.\nWe strive to help people get better at programming.\nWe will be providing example templates for other users to use and learn.")
            },
            _ if command.starts_with("help") => {
                println!("Here is a list of commands...");
                println!("-lua: the lua command starts the lua terminal and typing 'exit' will quit the lua terminal");
                println!("-rust: the rust command starts the rust terminal and typing 'exit' will quit the rust terminal");
                println!("-spin: displays a spinning donut animation");
            },
            _ if command.starts_with("spin") => {
                let mut sausage = sausage_3d::Sausage3D::new();
                sausage.spin();
            },

            _ if command.starts_with("obj") => {
                load_obj();
            }
            _ => println!("Command not found: {}", command),
        }
    }
}
