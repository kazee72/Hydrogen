use hydrogen::file_io;

fn main() {
    match file_io::run_compression("test2.txt") {
        Ok(()) => println!("Compression Successful"),
        Err(e) => eprintln!("Compression Failed: {}", e),
    };
    match file_io::run_decompression("test2.h2") {
        Ok(()) => println!("Decompression Successful"),
        Err(e) => eprintln!("Decompression Failed: {}", e),
    };
}