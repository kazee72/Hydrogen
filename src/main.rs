use std::io;

mod rle;
mod utils;


fn main() {
    let result: Result<(), io::Error> = rle::compress();

    match result {
        Ok(_) => println!("Status General: OK"),
        Err(e) => println!("Status: Error -> {}", e),
    }
}
