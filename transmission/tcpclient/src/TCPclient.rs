use serde::{Deserialize, Serialize};
use std::process::Command;
use std::net::TcpStream;
use std::io::BufReader;
use serde_json::Value;
use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::io::Read;
use std::env;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Settings {
    ip_recipient: String,
    file_path: String,
    encryption: String,
    port: String
}

fn main() {

    let current_dir = env::current_dir().expect("Unable to get the current directory");
    let current_dir_str = current_dir.to_str().expect("Unable to convert to string");
    let modified_dir = format!("{}settings.json", &current_dir_str[0..current_dir_str.len()-22]);

    let file = File::open(modified_dir).unwrap();
    let reader = BufReader::new(file);
    let settings_data: Settings = serde_json::from_reader(reader).unwrap();

    match TcpStream::connect((&settings_data.ip_recipient[..], settings_data.port.parse::<u16>().unwrap())) {
        Ok(mut stream) => {

            let data_ask: String = ("|ask|".to_owned() + &settings_data.ip_recipient + "#" + &settings_data.encryption).to_string();
            stream.write(&data_ask.as_bytes()).unwrap();

            let mut buffer = [0; 131072];
            let size = stream.read(&mut buffer).unwrap();
            let response = String::from_utf8_lossy(&buffer[..size]).to_string();

            let parts: Vec<&str> = response.split("#").collect();
            let concent = parts[0];
            let encryption = parts[1];
            let key = parts[2];
            let ip_last_connection = parts[3];

            if concent == "yes" {
                if encryption == "key"{
                    
                    let mut stream = TcpStream::connect((&settings_data.ip_recipient[..], settings_data.port.parse::<u16>().unwrap())).expect("Server Problem");

                    let pub_key_dir = format!("{}GenKey/keys_client/public.pem", &current_dir_str[0..current_dir_str.len()-9]);
                    let pub_key_path = Path::new(&pub_key_dir);
                    let mut file = File::create(pub_key_path).expect("Failed to create file");
                    file.write_all(key.as_bytes()).expect("Failed to write to file");

                    let file_dir = format!("{}{}", &current_dir_str[0..current_dir_str.len()-22], settings_data.file_path);
                    let file = File::open(file_dir).expect("Unable to open file");
                    let reader = BufReader::new(file);

                    let data: Value = serde_json::from_reader(reader).expect("Unable to read file");
                    
                    let json_string = serde_json::to_string(&data).expect("Unable to serialize JSON");
                    let dir_json_string = format!("{}GenKey/keys_client/data/data.txt", &current_dir_str[0..current_dir_str.len()-9]);
                    let path_json_string = Path::new(&dir_json_string);
                    let mut data_file = File::create(path_json_string).expect("Falied to create file");
                    data_file.write_all(json_string.as_bytes()).expect("Failed to write to file");

                    let encrypt_sh_file_dir = format!("{}GenKey/encryptK.sh", &current_dir_str[0..current_dir_str.len()-9]);
                    let encrypt_sh_file_path = Path::new(&encrypt_sh_file_dir);
                    Command::new("sh")
                            .arg(encrypt_sh_file_path)
                            .current_dir(encrypt_sh_file_path.parent().unwrap())
                            .spawn()
                            .expect("Failed to start the process");
                    
                    while fs::metadata(format!("{}GenKey/keys_client/data/data_encryption.txt", &current_dir_str[0..current_dir_str.len()-9])).is_err() {
                        std::thread::sleep(std::time::Duration::from_millis(50));
                    }
                    
                    let dir_encrypt_file = format!("{}GenKey/keys_client/data/data_encryption.txt", &current_dir_str[0..current_dir_str.len()-9]);
                    let read_content_file = fs::read(dir_encrypt_file).expect("Error");
                    let key_read_content_file = format!("|key|#{:?}#{}", read_content_file, ip_last_connection);

                    stream.write(key_read_content_file.as_bytes()).expect("Failed to write data to socket");

                    fs::remove_file(format!("{}GenKey/keys_client/public.pem", &current_dir_str[0..current_dir_str.len()-9])).expect("Error");
                    fs::remove_file(format!("{}GenKey/keys_client/data/data.txt", &current_dir_str[0..current_dir_str.len()-9])).expect("Error");
                    fs::remove_file(format!("{}GenKey/keys_client/data/data_encryption.txt", &current_dir_str[0..current_dir_str.len()-9])).expect("Error");

                } else if encryption == "without" && key == "False" {

                    let mut stream = TcpStream::connect((&settings_data.ip_recipient[..], settings_data.port.parse::<u16>().unwrap())).expect("Server Problem");
                    let file_dir = format!("{}{}", &current_dir_str[0..current_dir_str.len()-22], settings_data.file_path);
                    let file = File::open(file_dir).expect("Unable to open file");
                    let reader = BufReader::new(file);

                    let data: Value = serde_json::from_reader(reader).expect("Unable to read file");
                    
                    let json_string = serde_json::to_string(&data).expect("Unable to serialize JSON");
                    let new_json_string = format!("{}#{}",json_string, ip_last_connection);
                    stream.write(new_json_string.as_bytes()).expect("Failed to write data to socket");

                }
            } else if concent == "no" {
                println!("The shipment was cancelled");
            }

        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Completed");
}