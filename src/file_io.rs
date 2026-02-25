use std::fs::{self, File};
use std::io::Write;
use std::time::Instant;

use crate::huffman;



pub fn run_compression(file_name: String) {

    let path: String = format!("tests/input_files/{}", file_name);
    let file_binary: Vec<u8> = fs::read(path).unwrap();

    let start = Instant::now();

    let char_freq: Vec<(u8, u32)> = huffman::get_freq(&file_binary);

    let (compressed_bytes, padding) = huffman::compress(&file_binary, char_freq.clone());

    let duration = start.elapsed();

    let uncompressed_size_bytes: usize = file_binary.len();
    let compressed_size_bytes: usize = compressed_bytes.len();

    println!("Uncompressed Length: {} bytes", uncompressed_size_bytes);
    println!("Compressed Length: {} bytes", compressed_size_bytes);
    println!("Compression Ratio: {:.2}%", (compressed_size_bytes as f64 / file_binary.len() as f64) * 100.0);
    println!("Compression Time: {:.10} ms", duration.as_secs_f64() * 1000.0);

    write_compressed(compressed_bytes, padding, char_freq, file_name);

}



pub fn write_compressed(compressed_bytes: Vec<u8>, padding: u8, char_freq: Vec<(u8, u32)>, file_name: String) {

    let path: String = format!("tests/output_files/{}.h2", file_name.trim_end_matches(".txt"));

    let mut compressed_file: File = File::create(path).unwrap();

    compressed_file.write_all(&[padding]).unwrap();

    // write number of unique bytes
    compressed_file.write_all(&[char_freq.len() as u8]).unwrap();
    
    // write character + frequency
    for (byte, freq) in &char_freq {
        compressed_file.write_all(&[*byte]).unwrap();
        compressed_file.write_all(&freq.to_le_bytes()).unwrap();
    }

    // write actual data
    compressed_file.write_all(&compressed_bytes).unwrap();

}



pub fn read_header(file_name: String) -> (u8, Vec<(u8, u32)>, Vec<u8>) {

    let path: String = format!("tests/output_files/{}", file_name);

    let file_binary: Vec<u8> = fs::read(path).unwrap();

    let padding: u8 = file_binary[0];
    let unique_entries: u8 = file_binary[1];
    let mut char_freq: Vec<(u8, u32)> = Vec::new();

    let mut pos: usize = 2;

    for _ in 0..unique_entries {
        let char_byte: u8 = file_binary[pos];

        let mut freq: u32 = file_binary[pos + 1] as u32;
        freq += file_binary[pos + 2] as u32 * 256;
        freq += file_binary[pos + 3] as u32 * 65536;
        freq += file_binary[pos + 4] as u32 * 16777216;

        char_freq.push((char_byte, freq));

        pos += 5;
    }

    let compressed_data: Vec<u8> = file_binary[pos..].to_vec();

    return (padding, char_freq, compressed_data);   
}



pub fn run_decompression(file_name: String) {

    let start = Instant::now();

    let (padding, char_freq, compressed_data) = read_header(file_name.clone());

    let output: Vec<u8> = huffman::decode(padding, char_freq, compressed_data);

    let duration = start.elapsed();

    println!("Decompression Time: {:.10} ms", duration.as_secs_f64() * 1000.0);

    write_decompressed(output, file_name);

    let original: Vec<u8> = fs::read("tests/input_files/test2.txt".to_string()).unwrap();
    let decompressed: Vec<u8> = fs::read("tests/output_files/test2_decompressed.txt".to_string()).unwrap();
    println!("Match: {}", original == decompressed);
}



pub fn write_decompressed(input_bytes: Vec<u8>, file_name: String) {
    let mut file: File = File::create(format!("tests/output_files/{}_decompressed.txt", file_name.trim_end_matches(".h2"))).unwrap();
    file.write_all(&input_bytes).unwrap();
}