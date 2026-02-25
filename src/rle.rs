use std::fs::{ self, File};
use std::time::Instant;
use std::io::{self, Write};

pub fn compress(file_name: &str) -> io::Result<()> {
    let path = format!("tests/input_files/{file_name}");
    // Read file as binary
    let bytes: Vec<u8> = fs::read(path.to_string())?;
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
    let path: String = format!("tests/output_files/{}.h2", input_file_name.trim_end_matches(".txt"));
    // Create new file
    let mut file = File::create(path)?;

    // Write compressed data to new file
    for (byte, count) in compressed_bytes {
        file.write_all(&[*byte])?;
        file.write_all(&count.to_le_bytes())?;
    }

    Ok(())
}

pub fn decompress(file_name: &str) {
    let path: String = format!("tests/output_files/{file_name}");

    let bytes: Vec<u8> = fs::read(path.to_string()).unwrap();
    let mut out: Vec<u8> = Vec::new();

    let start = Instant::now();

    let mut i: usize = 0;

    while i + 5 <= bytes.len() {
        let character: u8 = bytes[i];
        let amount: [u8; 4] = bytes[i+1..i+5].try_into().unwrap();
        let count: u32 = u32::from_le_bytes(amount);

        for _ in 0..count {
            out.push(character);
        }

        i += 5;
    }

    let duration = start.elapsed();

    let _ = write_decompressed(&out, file_name.trim_end_matches(".txt"));
    
    println!("Decompression Time: {:.10} ms", duration.as_secs_f64() * 1000.0);

}

pub fn write_decompressed(decompressed_bytes: &Vec<u8>, input_file_name: &str) -> io::Result<()> {
    let path: String = format!("tests/output_files/{}_decompressed.txt", input_file_name.trim_end_matches(".h2"));
    // Create new file
    let mut file = File::create(path)?;

    // Write compressed data to new file
    for byte in decompressed_bytes {
        file.write_all(&[*byte])?;
    }

    Ok(())
}
