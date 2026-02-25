use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::time::Instant;

use crate::huffman;



pub fn run_compression(input_path: &str, output_path: &str) -> io::Result<()> {

    let file_binary = fs::read(input_path)?;

    let start = Instant::now();

    let char_freq  = huffman::get_freq(&file_binary);

    let (compressed_bytes, padding) = huffman::compress(&file_binary, &char_freq);

    let duration = start.elapsed();

    let uncompressed_size_bytes = file_binary.len();
    let compressed_size_bytes = compressed_bytes.len();

    println!("Uncompressed Length: {} bytes", uncompressed_size_bytes);
    println!("Compressed Length: {} bytes", compressed_size_bytes);
    println!("Compression Ratio: {:.2}%", (compressed_size_bytes as f64 / file_binary.len() as f64) * 100.0);
    println!("Compression Time: {:.10} ms", duration.as_secs_f64() * 1000.0);

    write_compressed(compressed_bytes, padding, &char_freq, output_path)?;

    Ok(())
}



pub fn write_compressed(compressed_bytes: Vec<u8>, padding: u8, char_freq: &HashMap<u8, u32>, path: &str) -> io::Result<()> {

    let mut compressed_file = File::create(path)?;

    compressed_file.write_all(&[padding])?;

    // write number of unique bytes
    compressed_file.write_all(&[(char_freq.len() - 1) as u8])?;
    
    // write character + frequency
    for (byte, freq) in char_freq {
        compressed_file.write_all(&[*byte])?;
        compressed_file.write_all(&freq.to_le_bytes())?;
    }

    // write actual data
    compressed_file.write_all(&compressed_bytes)?;

    Ok(())
}



pub fn read_header(path: &str) -> io::Result<(u8, HashMap<u8, u32>, Vec<u8>)> {

    let file_binary: Vec<u8> = fs::read(path)?;

    let padding: u8 = file_binary[0];
    let unique_entries = file_binary[1] as u16 + 1;
    let mut char_freq: HashMap<u8, u32> = HashMap::new();

    let mut pos: usize = 2;

    for _ in 0..unique_entries {
        let char_byte = file_binary[pos];

        let freq = u32::from_le_bytes([
            file_binary[pos + 1],
            file_binary[pos + 2],
            file_binary[pos + 3],
            file_binary[pos + 4],
        ]);

        char_freq.insert(char_byte, freq);

        pos += 5;
    }

    let compressed_data = file_binary[pos..].to_vec();

    Ok((padding, char_freq, compressed_data))
}



pub fn run_decompression(input_path: &str, output_path: &str) -> io::Result<()>{

    let start = Instant::now();

    let (padding, char_freq, compressed_data) = read_header(input_path)?;

    let output = huffman::decode(padding, &char_freq, compressed_data);

    let duration = start.elapsed();

    println!("Decompression Time: {:.10} ms", duration.as_secs_f64() * 1000.0);

    write_decompressed(output, output_path)?;

    Ok(())
}



pub fn write_decompressed(input_bytes: Vec<u8>, path: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&input_bytes)?;
    Ok(())
}