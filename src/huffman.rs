use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

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

pub fn compress() {

    let file_binary = read_file_binary(format!("tests/input_files/test2.txt")).unwrap();

    let char_freq: Vec<(u8, u32)> = get_freq(&file_binary);
    let root: Node = build_binary_tree(char_freq);

    let mut codes:  HashMap<u8, String> = HashMap::new();

    generate_codes(root, String::new(), &mut codes);

    let mut bytes_string: String = String::new();

    for byte in file_binary {
        bytes_string.push_str(codes.get(&byte).unwrap());
    }



    

}