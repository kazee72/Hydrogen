use std::fs;
use std::io;

pub fn read_file_binary(path: String) -> io::Result<Vec<u8>> {
    fs::read(path)
}