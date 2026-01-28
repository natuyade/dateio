use std::io;
use std::fs;
use std::io::*;

pub fn create_file() {
    
    let path = "pre_date.rs";
    
    if !std::path::Path::new(path).exists() {
        let mut file = fs::File::create("pre_date.rs").expect("Couldn't create file");
        // b""は文字リテラルをバイト列化させる
        // std::io,fsはバイト単位でファイルを読み書きする
        file.write_all(b"pub const pre_date:[&std; 2] = [\"{{DATE}}\", \"{{URL}}\"]").expect("Couldn't write file");
    } else {
        let mut file = fs::File::create("pre_date.rs").expect("Couldn't create file");
        file.write_all(b"pub const pre_date:[&std; 2] = [\"{{DATE}}\", \"{{URL}}\"]").expect("Couldn't write file");
    }
    
    println!("Enter date:");
    // 初期化型用意
    let mut date = String::new();
    // キーボード入力を受け取る.入力された文字をinputに書き込む.unwrap
    io::stdin().read_line(&mut date).unwrap();
    
    println!("Enter image url:");
    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap();
    
    // trimで改行無しの一行だけを読み取る
    println!("date: {}, img_url: {}",date.trim(),url.trim());
    
    // ReadWrite用のファイル用意
    let mut pre_file = fs::read_to_string("pre_date.rs").unwrap();
    
    // replace (from, to)
    pre_file = pre_file.replace("{{DATE}}", date.trim());
    pre_file = pre_file.replace("{{URL}}", url.trim());
    
    // writeは受け取った文字列を自動でバイト列に変換して書き込みを行うためbの必要なし
    fs::write("pre_date.rs", pre_file).unwrap();
    
    // 実行結果
    println!("{}",fs::read_to_string("pre_date.rs").unwrap());
}