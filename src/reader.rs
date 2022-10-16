use csv::{Error, Reader, StringRecord};
use encoding_rs::Encoding;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use std::fs::File;

use crate::args::KnownEncodings;

pub fn init_reader(
    transcoder: DecodeReaderBytes<File, Vec<u8>>,
) -> Reader<DecodeReaderBytes<File, Vec<u8>>> {
    csv::ReaderBuilder::new()
        .flexible(true)
        .delimiter(b'\t')
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
