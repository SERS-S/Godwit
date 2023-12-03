use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;
use std::env;
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Settings {
    ip_recipient: String,
    file_path: String,
    encryption: String
}

fn main() {
    let current_dir = env::current_dir().expect("Unable to get the current directory");
    let current_dir_str = current_dir.to_str().expect("Unable to convert to string");
    let modified_dir = format!("{}settings.json", &current_dir_str[0..current_dir_str.len()-22]);

    let file = File::open(modified_dir).unwrap();
    let reader = BufReader::new(file);
    let settings_data: Settings = serde_json::from_reader(reader).unwrap();

    let file_dir = format!("{}{}", &current_dir_str[0..current_dir_str.len()-22], settings_data.file_path);

    let mut stream = TcpStream::connect((&settings_data.ip_recipient[..], 8080)).expect("Failed to connect to server");

    let file = File::open(file_dir).expect("Unable to open file");
    let reader = BufReader::new(file);

    let data: Value = serde_json::from_reader(reader).expect("Unable to read file");
    
    let json_string = serde_json::to_string(&data).expect("Unable to serialize JSON");
    stream.write(json_string.as_bytes()).expect("Failed to write data to socket");
}