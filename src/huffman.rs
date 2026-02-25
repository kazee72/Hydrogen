use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;



pub struct Node {
    character: Option<u8>,
    frequency: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node {
    pub fn new_leaf(character: u8, frequency: u32) -> Self {
        Node { character: Some(character), frequency, left: None, right: None }
    }

    pub fn new_internal(left: Node, right: Node) -> Self {
        Node { 
            character: None,
            frequency: left.frequency + right.frequency,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)), 
        }
    }
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



pub fn get_freq(file_bytes: &[u8]) -> HashMap<u8, u32> {

    let mut char_freq: HashMap<u8, u32> = HashMap::new();

    for byte in file_bytes {
        *char_freq.entry(*byte).or_insert(0) += 1;
    }

    char_freq
}



pub fn build_binary_tree(char_freq: &HashMap<u8, u32>) -> Node {
    
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();

    for (&character, &frequency) in char_freq {
        let temp_node= Node::new_leaf(character, frequency);
        heap.push(temp_node);
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        let new_node = Node::new_internal(left, right);

        heap.push(new_node);
    }

    let tree_root = heap.pop().unwrap();

    tree_root
}



pub fn generate_codes(node: &Node, current_code: String, codes: &mut HashMap<u8, String>) {

    if let Some(ch) = node.character {
        codes.insert(ch, current_code);
        return;
    }

    if let Some(left) = &node.left {
        generate_codes(left, current_code.clone() + "0", codes);
    }

    if let Some(right) = &node.right {
        generate_codes(right, current_code + "1", codes);
    }

}



pub fn compress(file_binary: &[u8], char_freq: &HashMap<u8, u32>) -> (Vec<u8>, u8) {

    let root = build_binary_tree(char_freq);

    let mut codes:  HashMap<u8, String> = HashMap::new();

    generate_codes(&root, String::new(), &mut codes);

    let mut bytes_string = String::new();

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

    (compressed_bytes, padding_bits_amount)
}



pub fn decode(padding: u8, char_freq: &HashMap<u8, u32>, compressed_data: Vec<u8>) -> Vec<u8> {

    let binary_tree_root = build_binary_tree(char_freq);

    let mut current_node = &binary_tree_root;
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
            let bit = (byte >> i) & 1;
            // 1 = right, 0 = left
            if bit == 1 {
                current_node = current_node.right.as_ref().unwrap();
            } else {
                current_node = current_node.left.as_ref().unwrap();
            }

            if let Some(ch) = current_node.character {
                output.push(ch);
                current_node = &binary_tree_root;
            }
        }
    }

    output
}