use std::io;
use std::fs;
use std::io::*;

use crate::waiting_cmd::set_command;

pub fn change_path() {
    
    let path = "setting.ini";
    
    let mut file = fs::File::create("setting.ini").expect("Couldn't create file");
    file.write_all(b"[Settings]\npath = {{PATH}}").unwrap();
    
    println!("\nEnter premier folder path:");
    
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path).unwrap();
    
    let mut setting_file = fs::read_to_string(path).unwrap();
    
    setting_file = setting_file.replace("{{PATH}}", input_path.trim());
    
    fs::write(path, setting_file).unwrap();
    
    println!("\nDone.\n[Settings]\npath = {}",input_path.trim());
    
    set_command()
}
