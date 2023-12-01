use std::net::{TcpListener, TcpStream};
use local_ip::get as get_local_ip;
use std::io::{Read, Write};
use std::fs::File;
use serde_json;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::io::{BufReader};
use std::env;

#[derive(Deserialize)]
struct Data {
    count: String,
}

#[derive(Serialize, Deserialize)]
struct Settings {
    count: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { count: 0.to_string() }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).expect("Failed to read data from socket");

    let message = String::from_utf8_lossy(&buffer).to_string();
    // println!("Received message: {}", message);

    let json_data: serde_json::Value = serde_json::from_str(&message).expect("Failed to parse JSON");

    let current_dir = env::current_dir().expect("Не удалось получить текущий каталог");
    let current_dir_str = current_dir.to_str().expect("Не удалось преобразовать в строку");
    let modified_dir = format!("{}settings.json", &current_dir_str[0..current_dir_str.len()-9]);

    let path = Path::new(&modified_dir);

    let file = File::open(&path).expect("Не удалось открыть файл");
    let reader = BufReader::new(file);

    let data: Data = serde_json::from_reader(reader).expect("Не удалось прочитать JSON");

    let mut count: i32 = data.count.parse().expect("Не удалось преобразовать в число");

    let data_dir = format!("{}serverData/data_{}.json", &current_dir_str[0..current_dir_str.len()-9], count);
    let file = File::create(data_dir).expect("Failed to create file");
    serde_json::to_writer(&file, &json_data).expect("Failed to write data to file");

    let fil = fs::File::open(&path).expect("Не удалось открыть файл");
    let mut read = BufReader::new(fil);

    let mut settings: Settings = serde_json::from_reader(&mut read).expect("Не удалось прочитать JSON");

    count+=1;
    settings.count = count.to_string();

    let serialized = serde_json::to_string_pretty(&settings).expect("Не удалось преобразовать в JSON");
    fs::File::create(modified_dir).expect("Не удалось создать файл").write_all(serialized.as_bytes()).expect("Не удалось записать в файл");
}

fn main() {
    let local_ip = get_local_ip().unwrap();
    let listener = TcpListener::bind((local_ip, 8080)).expect("Failed to bind to address");
 
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected: {:?}", stream.peer_addr());
                handle_client(stream);
            },
            Err(e) => { println!("Error: {}", e); },
        }
    }
}