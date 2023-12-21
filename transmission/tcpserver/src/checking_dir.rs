pub mod check_dir {

use std::process::Command;
use std::io::BufWriter;
use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::env;
use std::fs;

pub fn check_directory() {

    let current_dir = env::current_dir().expect("Не удалось получить текущий каталог");
    let current_dir_str = current_dir.to_str().expect("Не удалось преобразовать в строку");
    let modified_dir = format!("{}serverData", &current_dir_str[0..current_dir_str.len()-9]);

    if !Path::new(&modified_dir).exists() {
        match fs::create_dir(&modified_dir) {
            Ok(_) => (),
            Err(e) => eprintln!("Failed to create directory: {}", e),
        }
    }

}

pub fn check_dir_keys(addr: String) {

    let current_dir = env::current_dir().expect("Unable to get the current directory");
    let current_dir_str = current_dir.to_str().expect("Unable to convert to string");
    let priv_file_dir = format!("{}GenKey/keys_server/private_{}.pem", &current_dir_str[0..current_dir_str.len()-9], &addr);
    let pub_file_dir = format!("{}GenKey/keys_server/public_{}.pem", &current_dir_str[0..current_dir_str.len()-9], &addr);
    let priv_file_puth = Path::new(&priv_file_dir);
    let pub_file_puth = Path::new(&pub_file_dir);

    if priv_file_puth.exists() && pub_file_puth.exists() {
        {}
    } else {
        let current_addr_path = format!("{}GenKey/keys_server/currentAddr.txt", &current_dir_str[0..current_dir_str.len()-9]);
        let file = File::create(current_addr_path).expect("Failed to open file");
        let mut writer = BufWriter::new(file);
        writer.write_all(addr.as_bytes()).expect("Failed to write to file");

        let gen_sh_file_dir = format!("{}GenKey/genK.sh", &current_dir_str[0..current_dir_str.len()-9]);
        let gen_sh_file_path = Path::new(&gen_sh_file_dir);
        Command::new("sh")
                .arg(gen_sh_file_path)
                .current_dir(gen_sh_file_path.parent().unwrap())
                .spawn()
                .expect("Failed to start the process");
    }

}

pub fn check_dir_decrypted() {

    let current_dir = env::current_dir().expect("Unable to get the current directory");
    let current_dir_str = current_dir.to_str().expect("Unable to convert to string");
    let decrypt_sh_file_dir = format!("{}GenKey/decryptK.sh", &current_dir_str[0..current_dir_str.len()-9]);
    let decrypt_sh_file_path = Path::new(&decrypt_sh_file_dir);
    Command::new("sh")
            .arg(decrypt_sh_file_path)
            .current_dir(decrypt_sh_file_path.parent().unwrap())
            .spawn()
            .expect("Failed to start the process");

}

}