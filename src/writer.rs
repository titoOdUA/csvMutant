use std::{fs::File, path::PathBuf};

use csv::{Writer, WriterBuilder};

pub fn init_writer(file_path: PathBuf) -> Writer<File> {
    WriterBuilder::new()
        .flexible(true)
        .from_path(file_path)
        .expect("Failed to create writer!")
}

pub fn write_to_file(writer: &mut Writer<File>, data: &[Vec<String>]) {
    data.iter()
        .for_each(|row| writer.write_record(row).expect("Failed to write row!"));
}
