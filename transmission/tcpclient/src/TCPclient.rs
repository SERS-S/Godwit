use std::net::TcpStream;
use std::io::Write;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
use std::env;

fn main() {
    let mut stream = TcpStream::connect("192.168.1.67:8080").expect("Failed to connect to server");

    let current_dir = env::current_dir().expect("Unable to get the current directory");
    let current_dir_str = current_dir.to_str().expect("Unable to convert to string");
    let modified_dir = format!("{}file.json", &current_dir_str[0..current_dir_str.len()-22]);

    let file = File::open(modified_dir).expect("Unable to open file");
    let reader = BufReader::new(file);

    let data: Value = serde_json::from_reader(reader).expect("Unable to read file");
    
    let json_string = serde_json::to_string(&data).expect("Unable to serialize JSON");
    stream.write(json_string.as_bytes()).expect("Failed to write data to socket");
}