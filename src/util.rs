pub fn hex_to_vec(hex: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for byte in String::from(hex).split(" ") {
        bytes.push(u8::from_str_radix(byte, 16).unwrap());
    }
    bytes
}
