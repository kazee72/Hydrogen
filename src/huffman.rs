use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

use crate::utils::read_file_binary;



// struct for Node in the binary tree
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



// get all frequencies of unique characters in a file
pub fn get_freq(file_bytes: &[u8]) -> Vec<(u8, u32)> {

    let mut unique_chars: Vec<u8> = Vec::new();
    let mut char_freq: Vec<(u8, u32)> = Vec::new();
    
    // get all the different characters of the file
    for byte in file_bytes {
        if !unique_chars.contains(&byte) {
            unique_chars.push(*byte);
        }
    }

    // filter to see how frequent each character appears in the input file
    for c in &unique_chars {
        let count: usize = file_bytes.iter().filter(|&&b| b == *c).count();
        char_freq.push((*c, count as u32));
    }

    return char_freq;
}



pub fn build_binary_tree(char_freq: Vec<(u8, u32)>) -> Node {
    
    // collection of all the nodes before creating actual binary tree
    let mut node_collection: BinaryHeap<Node> = BinaryHeap::new();

    // create a node for every character
    for (character, frequency) in char_freq {
        let temp_node: Node = Node {
            character: Some(character),
            frequency: frequency,
            left: None,
            right: None
        };

        node_collection.push(temp_node);
    }

    // build actual tree
    while node_collection.len() > 1 {
        // get two lowest frequency nodes
        let node_1: Node = node_collection.pop().unwrap();
        let node_2: Node = node_collection.pop().unwrap();

        // make new node with combined frequency of node_1 and node_2
        let new_node: Node = Node {
            character: None,
            frequency: node_1.frequency + node_2.frequency,
            // left and right point to the subnodes
            left: Some(Box::new(node_1)),
            right: Some(Box::new(node_2))
        };

        node_collection.push(new_node);
    }
    // get the root
    let tree_root: Node = node_collection.pop().unwrap();

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
    // create compressed file
    let mut compressed_file: File = File::create(path).unwrap();

    // write padding amount
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

pub fn run(file_name: String) {

    let path = format!("tests/input_files/{}", file_name);
    let file_binary = read_file_binary(path).unwrap();

    // start time
    let start = Instant::now();

    let char_freq: Vec<(u8, u32)> = get_freq(&file_binary);

    // compress data
    let (compressed_bytes, padding) = compress(&file_binary, char_freq.clone());

    // end time
    let duration = start.elapsed();

    let uncompressed_size_bytes: usize = file_binary.len();
    let compressed_size_bytes: usize = compressed_bytes.len();

    println!("Uncompressed Length: {} bytes", uncompressed_size_bytes);
    println!("Compressed Length: {} bytes", compressed_size_bytes);
    println!("Compression Ratio: {:.2}%", (compressed_size_bytes as f64 / file_binary.len() as f64) * 100.0);
    println!("Compression Time: {:.10} ms", duration.as_secs_f64() * 1000.0);

    // write the data to new file
    write_compressed(compressed_bytes, padding, char_freq, file_name);

}