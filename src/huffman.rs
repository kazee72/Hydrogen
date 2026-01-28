use crate::utils::read_file_binary;



pub fn get_freq(file_name: &str) -> Vec<u8> {
    let path: String = format!("tests/input_files/{file_name}");

    let bytes: Vec<u8> = read_file_binary(path.to_string()).unwrap();

    let mut unique_chars: Vec<u8> = Vec::new();
    let mut char_freq: Vec<(u8, u32)> = Vec::new();
    
    // get all the different characters of the file
    for byte in &bytes {
        if !unique_chars.contains(&byte) {
            unique_chars.push(*byte);
        }
    }

    // filter to see how frequent each character appears in the input file
    for c in &unique_chars {
        let count: usize = bytes.iter().filter(|&&b| b == *c).count();
        char_freq.push((*c, count as u32));
    }

    // sort char_freq according to the second pos in tuple (frequency of the character)
    char_freq.sort_by_key(|t| t.1);
    // reverse it so the most frequent character is first
    char_freq.reverse();
    
    // make new vec with only bytes of chars in sequence
    let char_seq: Vec<u8> = char_freq.iter().map(|(n, _)| *n).collect();

    return char_seq;
}