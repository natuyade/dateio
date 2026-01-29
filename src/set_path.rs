use std::fs;
use std::io;
use std::io::*;

use crate::waiting_cmd::set_command;

pub fn change_path() {
    let path = "setting.ini";

    let mut file = fs::File::create("setting.ini").expect("\nCouldn't create file\n");
    file.write_all(b"[Settings]\npath = {{PATH}}").unwrap();

    println!("\nEnter src folder path:");
    print!("user> ");

    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path).unwrap();

    let trimed_path = input_path.trim().trim_matches('"');

    let mut setting_file = fs::read_to_string(path).unwrap();

    setting_file = setting_file.replace("{{PATH}}", trimed_path);

    fs::write(path, setting_file).unwrap();

    println!(
        "\nDone:\n\x20\x20\x20\x20[Settings]\n\x20\x20\x20\x20path = {}\n",
        trimed_path
    );

    set_command()
}
