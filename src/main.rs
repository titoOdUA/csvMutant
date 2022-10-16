use std::fs::File;
mod args;
mod process_data;
mod reader;
mod writer;

fn main() {
    //parse_args and get file path
    let file_path = args::parse_args();
    let result_file_path = format!("result_{}", file_path);
    let file = File::open(file_path).expect("Failed to open file!");
    //decode file content to utf8, change separator
    let transcoder = reader::prepare_decoder(file);
    //init reader
    let rdr = reader::init_reader(transcoder);
    //init writer
    let mut writer = writer::init_writer(result_file_path);
    //read records from file
    let origin_rows = reader::read_rows(rdr);
    //convert records to raw data
    let mut raw_data_rows = process_data::get_raw_data(origin_rows);
    //process_data to apply changes if necessary
    process_data::change_na_to_dashes(&mut raw_data_rows);
    //write changed data to file
    writer::write_to_file(&mut writer, &raw_data_rows);
    writer.flush().expect("Failed to flush writer!");
}
