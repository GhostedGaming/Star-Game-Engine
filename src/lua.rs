use std::{io::Write, mem::replace};

use mlua::prelude::*;

use chrono::prelude::*;

use colored::Colorize;


pub fn start_lua() -> LuaResult<()> {
    let lua = Lua::new_with(
        mlua::StdLib::ALL_SAFE, 
        LuaOptions::default()
    )?;
    
    let globals = lua.globals();

    println!("\x1b[94mLua Terminal Started - Type 'exit' to return to main shell\x1b[0m");

    loop {

        let datetime: DateTime<Local> = Local::now();
        print!("\x1b[36mlua[{}]>\x1b[0m ", datetime.format("%H:%M:%S").to_string());
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let command = input.trim();
        
        if command == "exit" {
            break;
        }

         // Basic Colors
         let red = "\x1b[38;2;255;0;0m";
         let green = "\x1b[38;2;0;255;0m";
         let blue = "\x1b[38;2;0;0;255m";
 
         // Pastels
         let pastel_pink = "\x1b[38;2;255;182;193m";
         let pastel_blue = "\x1b[38;2;173;216;230m";
         let pastel_green = "\x1b[38;2;152;251;152m";
 
         // Neons
         let neon_pink = "\x1b[38;2;255;20;147m";
         let neon_green = "\x1b[38;2;57;255;20m";
         let neon_blue = "\x1b[38;2;31;81;255m";
 
         // Earth Tones
         let brown = "\x1b[38;2;139;69;19m";
         let forest_green = "\x1b[38;2;34;139;34m";
         let terracotta = "\x1b[38;2;204;78;51m";
 
         // Ocean Colors
         let aqua = "\x1b[38;2;0;255;255m";
         let teal = "\x1b[38;2;0;128;128m";
         let navy = "\x1b[38;2;0;0;128m";
 
         // Warm Colors
         let orange = "\x1b[38;2;255;165;0m";
         let coral = "\x1b[38;2;255;127;80m";
         let gold = "\x1b[38;2;255;215;0m";
 
         // Cool Colors
         let indigo = "\x1b[38;2;75;0;130m";
         let violet = "\x1b[38;2;238;130;238m";
         let turquoise = "\x1b[38;2;64;224;208m";
 
         // Jewel Tones
         let ruby = "\x1b[38;2;224;17;95m";
         let emerald = "\x1b[38;2;46;204;113m";
         let sapphire = "\x1b[38;2;15;82;186m";
 
         // Metallics
         let silver = "\x1b[38;2;192;192;192m";
         let bronze = "\x1b[38;2;205;127;50m";
         let copper = "\x1b[38;2;184;115;51m";
 
         // Unique Colors
         let lavender = "\x1b[38;2;230;230;250m";
         let mint = "\x1b[38;2;189;252;201m";
         let raspberry = "\x1b[38;2;227;11;93m";
         
         // Add syntax highlighting here
         let highlighted_command = command
             .replace("print(", format!("{}print\x1b[0m(", neon_blue).as_str())
             .replace("type(", format!("{}type\x1b[0m(", neon_green).as_str())
             .replace("tostring(", format!("{}tostring\x1b[0m(", aqua).as_str())
             .replace("tonumber(", format!("{}tonumber\x1b[0m(", orange).as_str())
             .replace("if", format!("{}if\x1b[0m(",turquoise ).as_str());
        
        let warn: LuaFunction = lua.create_function(|_, (msg,): (String,)| {
            let pre_text= "⚠ Warning".yellow();
            println!(r"{} {}", pre_text, msg.yellow());
            Ok(())
        })?;
            globals.set("warn", warn)?;
        
        match lua.load(command).exec() {
            Ok(_) => {},
            Err(e) => println!("\x1b[31mLua Error: {}\x1b[0m", e),
        }
    }

    Ok(())
}
