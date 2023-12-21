mod checking_dir;
use crate::checking_dir::check_dir::check_directory;
use crate::checking_dir::check_dir::check_dir_keys;
use crate::checking_dir::check_dir::check_dir_decrypted;

use std::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use local_ip::get as get_local_ip;
use std::io::{Read, Write};
use serde_json::from_str;
use std::io::BufReader;
use serde_json::Value;
use serde_json::json;
use std::path::Path;
use std::fs::File;
use serde_json;
use std::env;
use std::fs;



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
    let mut buffer = [0; 2048];
    let size = stream.read(&mut buffer).unwrap();
    let mut message = String::from_utf8_lossy(&buffer[..size]).to_string();

    if message.get(0..5).unwrap() == "|ask|" {

    let message = message.split_off(5);
    let parts: Vec<&str> = message.split("#").collect();
    let ip_address = parts[0];
    let encryption = parts[1];

    let current_dir = env::current_dir().expect("Unable to get the current directory");
    let current_dir_str = current_dir.to_str().expect("Unable to convert to string");
    let modified_dir = format!("{}/src/blackIPlist.json", &current_dir_str);
    let json_str = fs::read_to_string(modified_dir).expect("Unable to read file");
    let json: Value = from_str(&json_str).expect("Invalid JSON");
    let ip_list = json["ip_list"].as_array().unwrap();

    let list: Vec<String> = ip_list.iter().map(|s| s.to_string()).collect();
    let mut response = String::new();

    if list.contains(&ip_address.to_string()) {
        response+="no#";
    } else {
        response+="yes#";
    }

    if encryption == "True" {

        response+="key#";

        let str_addr: String = match stream.peer_addr() {
            Ok(socket_addr) => socket_addr.to_string(),
            Err(err) => format!("Error: {}", err),
        };
        
        let current_dir = env::current_dir().expect("Unable to get the current directory");
        let current_dir_str = current_dir.to_str().expect("Unable to convert to string");
        let priv_file_dir = format!("{}GenKey/keys_server/private_{}.pem", &current_dir_str[0..current_dir_str.len()-9], &str_addr);
        let pub_file_dir = format!("{}GenKey/keys_server/public_{}.pem", &current_dir_str[0..current_dir_str.len()-9], &str_addr);
        let _priv_file_puth = Path::new(&priv_file_dir);
        let pub_file_puth = Path::new(&pub_file_dir);

        check_dir_keys(str_addr);

        while fs::metadata(pub_file_puth).is_err() {
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        let mut pubkey_file = File::open(&pub_file_puth).expect("Не удалось открыть файл");
        let mut contents = String::new();
        pubkey_file.read_to_string(&mut contents).expect("Не удалось прочитать файл");

        response+=&contents;
        let responce_idaddr = format!("#{:?}", stream.peer_addr());
        response+=&responce_idaddr;

    } else if encryption == "False" {
        response+="without#False";
        let responce_idaddr = format!("#{:?}", stream.peer_addr());
        response+=&responce_idaddr;
    }
    stream.write_all(response.as_bytes()).unwrap();


    let current_data = fs::read_to_string(format!("{}/src/currentMass.json", &current_dir_str)).expect("Unable to read file");
    let mut json: Value = serde_json::from_str(&current_data).expect("JSON does not have correct format");
    let cur_ip_list = json["current_ip_list"].as_array_mut().unwrap();
    let mut vec_ip_list: Vec<String> = cur_ip_list.iter().map(|ip| ip.as_str().unwrap().to_string()).collect();
    
    vec_ip_list.push(format!("{:?}", stream.peer_addr()));

    let new_ip_list: Vec<Value> = vec_ip_list.into_iter().map(Value::String).collect();
    json["current_ip_list"] = json!(new_ip_list);
    let new_json = serde_json::to_string_pretty(&json).expect("Failed to serialize JSON");
    fs::write(format!("{}/src/currentMass.json", &current_dir_str), new_json).expect("Unable to write file");

    } else {

        if message.get(0..5) == Some("|key|") {

            let parts: Vec<&str> = message.split("#").collect();
            let _key_agre = parts[0];
            let data_str = parts[1];
            let ip_last_connection = parts[2][3..parts[2].len()-1].to_string();
            let modified_ip_last_connection = ip_last_connection.replace(".", "_");

            let current_dir = env::current_dir().expect("Unable to get the current directory");
            let current_dir_str = current_dir.to_str().expect("Unable to convert to string");
            let current_data = fs::read_to_string(format!("{}/src/currentMass.json", &current_dir_str)).expect("Unable to read file");
            let mut json: Value = serde_json::from_str(&current_data).expect("JSON does not have correct format");
            let cur_ip_list = json["current_ip_list"].as_array_mut().unwrap();
            let mut vec_ip_list: Vec<String> = cur_ip_list.iter().map(|ip| ip.as_str().unwrap().to_string()).collect();

            if vec_ip_list.contains(&format!("Ok({})", ip_last_connection)) {

                let data_bytes: Vec<u8> = data_str
                    .trim_matches(|c| c == '[' || c == ']')
                    .split(", ")
                    .map(|s| s.parse().expect("Failed to parse u8"))
                    .collect();
                
                let current_dir = env::current_dir().expect("Unable to get the current directory");
                let current_dir_str = current_dir.to_str().expect("Unable to convert to string");
                let dir_get_encrypted_data = format!("{}GenKey/keys_server/data/data_{}.txt", &current_dir_str[0..current_dir_str.len()-9], modified_ip_last_connection);
                let path_get_encrypted_data = Path::new(&dir_get_encrypted_data);
                let mut get_data_file = File::create(path_get_encrypted_data).expect("Falied to create file");
                get_data_file.write_all(&data_bytes).expect("Failed to write to file");

                let dir_get_decrypted_data = format!("{}GenKey/keys_server/data/last_ip.txt", &current_dir_str[0..current_dir_str.len()-9]);
                let path_get_decrypted_data = Path::new(&dir_get_decrypted_data);
                let mut get_data_decrypt_file = File::create(path_get_decrypted_data).expect("Falied to create file");
                get_data_decrypt_file.write_all(ip_last_connection.as_bytes()).expect("Failed to write to file");

                check_dir_decrypted();

                while fs::metadata(format!("{}GenKey/keys_server/data/data_{}_decrypted.txt", &current_dir_str[0..current_dir_str.len()-9], modified_ip_last_connection)).is_err() {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }

                let save_json_dir = format!("{}GenKey/keys_server/data/data_{}_decrypted.txt", &current_dir_str[0..current_dir_str.len()-9], modified_ip_last_connection);
                let save_json_path = Path::new(&save_json_dir);
                let mut js_file = File::open(save_json_path).expect("Error");
                let mut contentss = String::new(); 
                js_file.read_to_string(&mut contentss).expect("Error");
                let json_data: Value = serde_json::from_str(&contentss).expect("Error");

                let modified_dir = format!("{}count.json", &current_dir_str[0..current_dir_str.len()-9]);
                let path = Path::new(&modified_dir);
                let file = File::open(&path).expect("Не удалось открыть файл");
                let reader = BufReader::new(file);
                let data: Data = serde_json::from_reader(reader).expect("Не удалось прочитать JSON");
                let mut count: i32 = data.count.parse().expect("Не удалось преобразовать в число");

                check_directory();
        
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

                fs::remove_file(format!("{}GenKey/keys_server/private_{}.pem", &current_dir_str[0..current_dir_str.len()-9], ip_last_connection)).expect("Error");
                fs::remove_file(format!("{}GenKey/keys_server/public_{}.pem", &current_dir_str[0..current_dir_str.len()-9], ip_last_connection)).expect("Error");

                fs::remove_file(format!("{}GenKey/keys_server/data/data_{}_decrypted.txt", &current_dir_str[0..current_dir_str.len()-9], modified_ip_last_connection)).expect("Error");
                fs::remove_file(format!("{}GenKey/keys_server/data/data_{}.txt", &current_dir_str[0..current_dir_str.len()-9], modified_ip_last_connection)).expect("Error");

                fs::remove_file(format!("{}GenKey/keys_server/data/last_ip.txt", &current_dir_str[0..current_dir_str.len()-9])).expect("Error");

                let str_ip_last_connection = format!("Ok({})", ip_last_connection);
                vec_ip_list.retain(|x| x != &str_ip_last_connection);

                let new_ip_list: Vec<Value> = vec_ip_list.into_iter().map(Value::String).collect();
                json["current_ip_list"] = json!(new_ip_list);
                let new_json = serde_json::to_string_pretty(&json).expect("Failed to serialize JSON");
                fs::write(format!("{}/src/currentMass.json", &current_dir_str), new_json).expect("Unable to write file");

            } else {}

        } else {

            let parts: Vec<&str> = message.split("#").collect();
            let message = parts[0];
            let ip_last_connection = parts[1].to_string();

            let current_dir = env::current_dir().expect("Unable to get the current directory");
            let current_dir_str = current_dir.to_str().expect("Unable to convert to string");
            let current_data = fs::read_to_string(format!("{}/src/currentMass.json", &current_dir_str)).expect("Unable to read file");
            let mut json: Value = serde_json::from_str(&current_data).expect("JSON does not have correct format");
            let cur_ip_list = json["current_ip_list"].as_array_mut().unwrap();
            let mut vec_ip_list: Vec<String> = cur_ip_list.iter().map(|ip| ip.as_str().unwrap().to_string()).collect();

            if vec_ip_list.contains(&ip_last_connection) {

                let json_data: serde_json::Value = serde_json::from_str(&message).expect("Failed to parse JSON");

                let current_dir = env::current_dir().expect("Не удалось получить текущий каталог");
                let current_dir_str = current_dir.to_str().expect("Не удалось преобразовать в строку");
                let modified_dir = format!("{}count.json", &current_dir_str[0..current_dir_str.len()-9]);
        
                let path = Path::new(&modified_dir);
        
                let file = File::open(&path).expect("Не удалось открыть файл");
                let reader = BufReader::new(file);
        
                let data: Data = serde_json::from_reader(reader).expect("Не удалось прочитать JSON");
        
                let mut count: i32 = data.count.parse().expect("Не удалось преобразовать в число");
        
                check_directory();
        
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

                vec_ip_list.retain(|x| x != &ip_last_connection);

                let new_ip_list: Vec<Value> = vec_ip_list.into_iter().map(Value::String).collect();
                json["current_ip_list"] = json!(new_ip_list);
                let new_json = serde_json::to_string_pretty(&json).expect("Failed to serialize JSON");
                fs::write(format!("{}/src/currentMass.json", &current_dir_str), new_json).expect("Unable to write file");

            } else {}
        }
    }
 
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