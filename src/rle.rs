use crate::utils::read_file_binary;
use std::fs::File;
use std::time::Instant;
use std::io::{self, Write};

pub fn compress() -> io::Result<()> {
    let file_name = "test.txt";
    let path = format!("tests/input_files/{file_name}");
    // Read file as binary
    let bytes: Vec<u8> = read_file_binary(path.to_string())?;
    let mut compressed: Vec<(u8, u32)> = Vec::new();
    let mut i: usize = 0;

    let start = Instant::now();

    // Main compression algorithm
    while i < bytes.len() {
        let current: u8 = bytes[i];
        let mut counter: usize = 1;

        while i + counter< bytes.len() && bytes[i + counter] == current {
            counter += 1;
        }

        compressed.push((current, counter as u32));
        i += counter;
    }

    let duration = start.elapsed();

    let uncompressed_size_bytes: usize = bytes.len();
    let compressed_size_bytes: usize = compressed.len() * 5;

    println!("Uncompressed Length: {} bytes", uncompressed_size_bytes);
    println!("Compressed Length: {} bytes", compressed_size_bytes);
    println!("Compression Ratio: {:.2}%", (compressed_size_bytes as f64 / bytes.len() as f64) * 100.0);
    println!("Compression Time: {:.10} ms", duration.as_secs_f64() * 1000.0);
    match write_compressed(&compressed, file_name) {
        Ok(_) => println!("Status File write: OK"),
        Err(e) => println!("Status File write: Error - {}", e),
    }

    Ok(())
}

pub fn write_compressed(compressed_bytes: &Vec<(u8, u32)>, input_file_name: &str) -> io::Result<()> {
    let path: String = format!("tests/output_files/{}.h2", input_file_name);
    // Create new file
    let mut file = File::create(path)?;

    // Write compressed data to new file
    for (byte, count) in compressed_bytes {
        file.write_all(&[*byte])?;
        file.write_all(&count.to_le_bytes())?;
    }

    Ok(())
}