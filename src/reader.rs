use csv::{Error, Reader, StringRecord};
use encoding_rs::Encoding;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use std::io::{self, prelude::*, BufReader};
use std::{fs::File, vec};

use crate::args::KnownEncodings;

pub fn init_reader(
    transcoder: DecodeReaderBytes<File, Vec<u8>>,
    separator: u8,
) -> Reader<DecodeReaderBytes<File, Vec<u8>>> {
    csv::ReaderBuilder::new()
        .flexible(true)
        .delimiter(separator)
        .has_headers(false)
        .from_reader(transcoder)
}

pub fn prepare_decoder(
    file: File,
    encoding: Option<KnownEncodings>,
) -> DecodeReaderBytes<File, Vec<u8>> {
    let source_file_encoding = match encoding {
        Some(e) => match e {
            KnownEncodings::Utf8 => Encoding::for_label(b"utf-8"),
            KnownEncodings::Utf16 => Encoding::for_label(b"utf-16le"),
        },
        None => Encoding::for_label(b"utf-8"),
    };

    DecodeReaderBytesBuilder::new()
        .encoding(source_file_encoding)
        .build(file)
}

pub fn read_rows(
    mut reader: Reader<DecodeReaderBytes<File, Vec<u8>>>,
) -> Vec<Result<StringRecord, Error>> {
    reader
        .records()
        .collect::<Vec<Result<StringRecord, Error>>>()
}

pub fn detect_separator(file: &mut File) -> u8 {
    let mut reader = BufReader::new(file);
    let mut line_buffer = String::new();
    reader.read_line(&mut line_buffer).expect("Can't read line");

    let known_separators = vec![',', ';', '\t'];
    let mut probable_separator: Option<char> = None;
    let mut number_of_elements = 0;

    known_separators.iter().for_each(|separator| {
        let n = line_buffer.split(*separator).count();
        if n > number_of_elements {
            probable_separator = Some(*separator);
            number_of_elements = n;
        }
    });

    //return reading possition to the start of the file
    reader
        .seek(io::SeekFrom::Start(0))
        .expect("Failed to return reader position");
    *probable_separator.get_or_insert(';') as u8
}
