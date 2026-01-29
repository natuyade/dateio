use std::fs;
use std::io;
use std::io::*;
use std::path::PathBuf;

use crate::waiting_cmd::set_command;

pub fn create_file() {
    // .exists()でcurrent dirにパスのものが存在するかを検知
    if std::path::Path::new("setting.ini").exists() {
        let settings = fs::read_to_string("setting.ini").unwrap();

        // ファイル操作にPathBufを使うので初期化
        let mut path: Option<PathBuf> = None;

        // SettingFileからpathの設定を探索
        for line in settings.lines() {
            // そのlineの始めの"="の左右の値を取得する
            if let Some((key, value)) = line.split_once("=") {
                // trimで余白を消し必要な項目を見つける
                if key.trim() == "path" {
                    // 必要なkeyが見つかったら値をとりPathBuf(パス型)へ変換
                    path = Some(PathBuf::from(value.trim()));
                }
            }
        }

        // Some()のままなので値取り出し
        let folder_path = path.unwrap();

        if !folder_path.exists() {
            // allは複数のdirを作れる, allがないと単体しか作れないのでエラー
            fs::create_dir_all(&folder_path).unwrap();
        }

        let file_name = "pre_date.rs";

        let file_path = folder_path.join(file_name);

        /*
         * path = の部分が空白の場合,無が入ったfolder_path(PathBuf)を受け取るが
         * file_pathがあとからjoinしているため相対パスでfileが作られる
         */

        let mut file = fs::File::create(&file_path).expect("\nCouldn't create file\n");
        // b""は文字リテラルをバイト列化させる
        // std::io,fsはバイト単位でファイルを読み書きする
        file.write_all(b"pub const pre_date:[&str; 2] = [\"{{DATE}}\", \"{{URL}}\"];")
            .expect("\nCouldn't write\n");

        println!("\nEnter date:");
        // 初期化型用意
        let mut date = String::new();
        // キーボード入力を受け取る.入力された文字をinputに書き込む.unwrap
        io::stdin().read_line(&mut date).unwrap();

        println!("\nEnter image url:");
        let mut url = String::new();
        io::stdin().read_line(&mut url).unwrap();

        // trimで半角スペース,\t,\n,\rの空白を消せる
        println!(
            "\nDone:\n\x20\x20\x20\x20date: {}, img_url: {}\n",
            date.trim(),
            url.trim()
        );

        // ReadWrite用のファイル用意
        let mut pre_file = fs::read_to_string(&file_path).unwrap();

        // replace (from, to)
        pre_file = pre_file.replace("{{DATE}}", date.trim());
        pre_file = pre_file.replace("{{URL}}", url.trim());

        // writeは受け取った文字列を自動でバイト列に変換して書き込みを行うためbの必要なし
        fs::write(&file_path, pre_file).unwrap();
    } else {
        println!("\nSettingFile not found,\nPlease use path command first.\n");
        set_command()
    }
}
