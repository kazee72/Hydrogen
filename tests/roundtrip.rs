use hydrogen::huffman;

#[test]
fn roundtrip_text() {
    let input = b"hello world";
    let char_freq = huffman::get_freq(input);
    let (compressed, padding) = huffman::compress(input, &char_freq);
    let output = huffman::decode(padding, &char_freq, compressed);
    assert_eq!(input.to_vec(), output);
}