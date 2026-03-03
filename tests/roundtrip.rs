use hydrogen::huffman;

fn assert_roundtrip(input: &[u8]) {
    let char_freq = huffman::get_freq(input);
    let (compressed, padding) = huffman::compress(input, &char_freq);
    let output = huffman::decode(padding, &char_freq, compressed);
    assert_eq!(input.to_vec(), output);
}


#[test]
fn roundtrip_text() {
    assert_roundtrip(b"hello world");
}

#[test]
fn roundtrip_single_byte() {
    assert_roundtrip(b"a");
}

#[test]
fn roundtrip_repeated_byte() {
    assert_roundtrip(b"aaaaaaa");
}

#[test]
fn roundtrip_empty() {
    assert_roundtrip(b"");
}

#[test]
fn roundtrip_all_256_bytes() {
    let input: Vec<u8> = (0..=255).collect();
    assert_roundtrip(&input);
}

#[test]
fn roundtrip_two_unique() {
    assert_roundtrip(b"ababab");
}