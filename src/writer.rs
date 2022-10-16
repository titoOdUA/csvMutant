use std::fs::File;

use csv::{Writer, WriterBuilder};

pub fn init_writer(file_path: String) -> Writer<File> {
    let mut path = String::new();
    if file_path.contains(".\\") {
        path.push_str(&file_path.replacen(".\\", "", 1));
    } else {
        path = file_path;
    }
    println!("result path is: {}", path);
    WriterBuilder::new().flexible(true).from_path(path).unwrap()
}

pub fn write_to_file(writer: &mut Writer<File>, data: &[Vec<String>]) {
    data.iter()
        .for_each(|row| writer.write_record(row).expect("Failed to write row!"));
}
