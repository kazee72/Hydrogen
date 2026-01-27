use std::io;

mod rle;
mod utils;


fn main() {
    let result: Result<(), io::Error> = rle::compress("test.txt");
    let result_decompress: Result<(), io::Error> = Ok(rle::decompress("test.txt.h2"));

    match result {
        Ok(_) => println!("Status General: OK"),
        Err(e) => println!("Status: Error -> {}", e),
    }

    match result_decompress {
        Ok(_) => println!("Status General: OK"),
        Err(e) => println!("Status: Error -> {}", e),
    }
}
