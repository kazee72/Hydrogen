use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

use crate::utils::read_file_binary;



pub struct Node {
    character: Option<u8>,
    frequency: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}



// --- turn MAX-Heap into MIN-Heap ---
// we do this by comparing the other with self instead of self with other
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}
// these need to be implemented for Ord to work for Node
// that is the case because Ord needs PartialOrd and Eq which in turns needs PartialEq
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl Eq for Node {}



pub fn get_freq(file_bytes: &[u8]) -> Vec<(u8, u32)> {

    let mut char_freq: HashMap<u8, u32> = HashMap::new();

    for byte in file_bytes {
        *char_freq.entry(*byte).or_insert(0) += 1;
    }

    return char_freq.into_iter().collect();
}



pub fn build_binary_tree(char_freq: Vec<(u8, u32)>) -> Node {
    
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();

    for (character, frequency) in char_freq {
        let temp_node: Node = Node {
            character: Some(character),
            frequency: frequency,
            left: None,
            right: None
        };

        heap.push(temp_node);
    }

    // build actual tree
    while heap.len() > 1 {
        let node_1: Node = heap.pop().unwrap();
        let node_2: Node = heap.pop().unwrap();

        let new_node: Node = Node {
            character: None,
            frequency: node_1.frequency + node_2.frequency,
            left: Some(Box::new(node_1)),
            right: Some(Box::new(node_2))
        };

        heap.push(new_node);
    }

    let tree_root: Node = heap.pop().unwrap();

    return tree_root;

}

pub fn generate_codes(node: Node, current_code: String, codes: &mut HashMap<u8, String>) {

    if node.character.is_some() {
        codes.insert(node.character.unwrap(), current_code);
        return;
    }

    if node.left.is_some() {
        generate_codes(*node.left.unwrap(), current_code.clone() + "0", codes);
    }

    if node.right.is_some() {
        generate_codes(*node.right.unwrap(), current_code + "1", codes);
    }

}

pub fn compress(file_binary: &[u8], char_freq: Vec<(u8, u32)>) -> (Vec<u8>, u8) {

    let root: Node = build_binary_tree(char_freq);

    let mut codes:  HashMap<u8, String> = HashMap::new();

    generate_codes(root, String::new(), &mut codes);

    let mut bytes_string: String = String::new();

    for byte in file_binary {
        bytes_string.push_str(codes.get(&byte).unwrap());
    }

    let mut compressed_bytes: Vec<u8> = Vec::new();
    let mut padding_bits_amount: u8 = 0;
    
    for i in  (0..bytes_string.len()).step_by(8) {
        let mut bytestring: String;
        if i + 8 > bytes_string.len() {
            bytestring = bytes_string[i..bytes_string.len()].to_string();
            padding_bits_amount = 8 - bytestring.len() as u8;
            bytestring = format!("{:0<8}", bytestring);
        } else {
            bytestring = bytes_string[i..i + 8].to_string();
        }
        compressed_bytes.push(u8::from_str_radix(&bytestring, 2).unwrap());
    }

    return (compressed_bytes, padding_bits_amount);
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



pub fn run_compression(file_name: String) {

    let path: String = format!("tests/input_files/{}", file_name);
    let file_binary: Vec<u8> = read_file_binary(path).unwrap();

    let start = Instant::now();

    let char_freq: Vec<(u8, u32)> = get_freq(&file_binary);

    let (compressed_bytes, padding) = compress(&file_binary, char_freq.clone());

    let duration = start.elapsed();

    let uncompressed_size_bytes: usize = file_binary.len();
    let compressed_size_bytes: usize = compressed_bytes.len();

    println!("Uncompressed Length: {} bytes", uncompressed_size_bytes);
    println!("Compressed Length: {} bytes", compressed_size_bytes);
    println!("Compression Ratio: {:.2}%", (compressed_size_bytes as f64 / file_binary.len() as f64) * 100.0);
    println!("Compression Time: {:.10} ms", duration.as_secs_f64() * 1000.0);

    write_compressed(compressed_bytes, padding, char_freq, file_name);

}



pub fn read_header(file_name: String) -> (u8, Vec<(u8, u32)>, Vec<u8>) {

    let path: String = format!("tests/output_files/{}", file_name);

    let file_binary: Vec<u8> = read_file_binary(path).unwrap();

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



pub fn decode(file_name: String) -> Vec<u8> {
    let (padding, char_freq, compressed_data) = read_header(file_name);

    let binary_tree_root: Node = build_binary_tree(char_freq);

    let mut current_node: &Node = &binary_tree_root;
    let mut output: Vec<u8> = Vec::new();

    for (byte_indx, byte) in compressed_data.iter().enumerate() {

        let bits_to_read;

        // ignore padding bits on the last byte
        if byte_indx == compressed_data.len() - 1 {
            bits_to_read = 8 - padding;
        } else {
            bits_to_read = 8;
        };


        for i in (8 - bits_to_read..8).rev() {
            let bit: u8 = (byte >> i) & 1;
            // 1 = right, 0 = left
            if bit == 1 {
                current_node = current_node.right.as_ref().unwrap();
            } else {
                current_node = current_node.left.as_ref().unwrap();
            }

            if current_node.character.is_some() {
                output.push(current_node.character.unwrap());
                current_node = &binary_tree_root;
            }
        }
    }

    return output;
}



pub fn write_decompressed(input_bytes: Vec<u8>, file_name: String) {
    let mut file: File = File::create(format!("tests/output_files/{}_decompressed.txt", file_name.trim_end_matches(".h2"))).unwrap();
    file.write_all(&input_bytes).unwrap();
}



pub fn run_decompression(file_name: String) {

    let start = Instant::now();

    let output: Vec<u8> = decode(file_name.clone());

    let duration = start.elapsed();

    println!("Decompression Time: {:.10} ms", duration.as_secs_f64() * 1000.0);

    write_decompressed(output, file_name);

    let original = read_file_binary("tests/input_files/test2.txt".to_string()).unwrap();
    let decompressed = read_file_binary("tests/output_files/test2_decompressed.txt".to_string()).unwrap();
    println!("Match: {}", original == decompressed);
}