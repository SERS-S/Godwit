use serde::{Deserialize};
use std::path::Path;
use std::io::{BufReader};
use std::fs::File;
use std::env;

#[derive(Deserialize)]
struct Data {
    count: String,
}

fn main() {

    let current_dir = env::current_dir().expect("Не удалось получить текущий каталог");
    let current_dir_str = current_dir.to_str().expect("Не удалось преобразовать в строку");
    let modified_dir = format!("{}settings.json", &current_dir_str[0..current_dir_str.len()-9]);

    let path = Path::new(&modified_dir);

    let file = File::open(&path).expect("Не удалось открыть файл");
    let reader = BufReader::new(file);

    let data: Data = serde_json::from_reader(reader).expect("Не удалось прочитать JSON");

    let count: i32 = data.count.parse().expect("Не удалось преобразовать в число");

    println!("Значение поля count: {}", count);
}