use csv::{Error, Reader, StringRecord};
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use std::fs::File;

pub fn init_reader(
    transcoder: DecodeReaderBytes<File, Vec<u8>>,
) -> Reader<DecodeReaderBytes<File, Vec<u8>>> {
    csv::ReaderBuilder::new()
        .flexible(true)
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(transcoder)
}

pub fn prepare_decoder(file: File) -> DecodeReaderBytes<File, Vec<u8>> {
    DecodeReaderBytesBuilder::new()
        .encoding(Some(encoding_rs::UTF_16LE))
        .build(file)
}

pub fn read_rows(
    mut reader: Reader<DecodeReaderBytes<File, Vec<u8>>>,
) -> Vec<Result<StringRecord, Error>> {
    reader
        .records()
        .collect::<Vec<Result<StringRecord, Error>>>()
}
