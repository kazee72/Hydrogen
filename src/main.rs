use std::io;

mod rle;
mod huffman;
mod file_io;


fn main() {
    //run_rle();
    run_huffman();
}
#[allow(dead_code)]
fn run_rle() {
    let result: Result<(), io::Error> = rle::compress("test.txt");
    let result_decompress: Result<(), io::Error> = Ok(rle::decompress("test.h2"));

    match result {
        Ok(_) => println!("Status General: OK"),
        Err(e) => println!("Status: Error -> {}", e),
    }

    match result_decompress {
        Ok(_) => println!("Status General: OK"),
        Err(e) => println!("Status: Error -> {}", e),
    }
}

fn run_huffman() {
    file_io::run_compression("test2.txt".to_string());
    file_io::run_decompression("test2.h2".to_string());
}
