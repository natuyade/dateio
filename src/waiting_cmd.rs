use std::io;
use std::io::*;

use crate::create_predate::create_file;
use crate::set_path::change_path;

pub fn set_command() {
    // 条件なしの無限loop
    loop {
        println!("Type command.(or type help)");
        print!("user> ");
        io::stdout().flush().unwrap();
<<<<<<< HEAD

        let mut input_cmd = String::new();
        stdin().read_line(&mut input_cmd).unwrap();

        let cmd = input_cmd.trim();

        match cmd {
            "path" => change_path(),
            "set" => {
                create_file();
                break;
            }
            "quit" => break,
            "help" => println!(
                "\nCommand list:\n\x20\x20\x20\x20path: set path.\n\x20\x20\x20\x20set: set date and imgsrc for premier.\n\x20\x20\x20\x20quit: close terminal.\n\x20\x20\x20\x20help: this command.\n"
            ),
            _ => println!("Unknown command"),
        };
=======
        
            let mut input_cmd = String::new();
            stdin().read_line(&mut input_cmd).unwrap();
            
            let cmd = input_cmd.trim();
            
            match cmd {
                "path" => {
                    change_path()
                },
                "set" => {
                    create_file();
                    break
                },
                "quit" => break,
                "help" => println!("\nCommand list:\n\x20\x20\x20\x20path: set path.\n\x20\x20\x20\x20set: set date and imgsrc for premier.\n\x20\x20\x20\x20quit: close terminal.\n\x20\x20\x20\x20help: this command.\n"),
                _ => println!("Unknown command")
            };
>>>>>>> 57b2d1d439931814285e81b68f5cf2d465d1ba93
    }
}
