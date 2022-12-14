use std::fs::File;

use args::Commands;
use clap::Parser;
mod args;
mod process_data;
mod reader;
mod writer;

fn main() {
    //parse_args and get file path
    let args = args::Cli::parse();
    let file_path = args.path;
    let mut file = File::open(&file_path).expect("Failed to open file!");
    //determine location and name for the result file, location == location of the original file
    let result_file_path = match file_path.parent() {
        Some(path) => {
            let filename = file_path.file_name().unwrap().to_str().unwrap();
            path.join(String::from("result_") + filename)
        }
        None => panic!("Can't determine file path for result file!"),
    };

    //try to detect separator or rollback to default ;
    let separator = reader::detect_separator(&mut file);
    //decode file content to utf8, change separator
    let transcoder = reader::prepare_decoder(file, args.encoding);
    //init reader
    let rdr = reader::init_reader(transcoder, separator);
    //init writer
    let mut writer = writer::init_writer(result_file_path);
    //read records from file
    let origin_rows = reader::read_rows(rdr);
    //convert records to raw data
    let mut raw_data_rows = process_data::get_raw_data(&origin_rows);

    //main logic block of the application
    match &args.command {
        Some(command) => match command {
            Commands::DeleteRows { n } => {
                raw_data_rows.drain(0..*n as usize);
            }
            Commands::ReplaceNA { val } => {
                process_data::change_na_values(&mut raw_data_rows, val);
            }
            Commands::ChangeDatesFormat {
                format,
                dates_row_number,
            } => process_data::change_dates_format(&mut raw_data_rows, format, *dates_row_number),
        },
        None => {
            println!("Command was not specified, executing the default behaviour.\nWhich is to replace n/a values with - symbol");
            process_data::change_na_values(&mut raw_data_rows, "-");
        }
    }

    //process_data to apply changes if necessary
    //write changed data to file
    writer::write_to_file(&mut writer, &raw_data_rows);
    writer.flush().expect("Failed to flush writer!");
}
