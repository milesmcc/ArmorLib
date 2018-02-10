/// This file defines several low-level utility functions for general use by ArmorLib.

use std::num::ParseIntError;

pub fn hex_to_vec(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut bytes: Vec<u8> = Vec::new();
    for byte in String::from(hex).split(" ") {
        bytes.push(u8::from_str_radix(byte, 16)?);
    }
    Ok(bytes)
}

pub fn u8s_to_u16(first: u8, second: u8) -> u16 {
    let first: u16 = (first as u16) << 8;
    let second: u16 = second as u16;

    first | second
}

pub fn u8s_to_u32(first: u8, second: u8, third: u8, fourth: u8) -> u32 {
    let first: u32 = (first as u32) << 24;
    let second: u32 = (second as u32) << 16;
    let third: u32 = (third as u32) << 8;
    let fourth: u32 = fourth as u32;

    first | second | third | fourth
}

#[cfg(test)]
mod tests {
    use ::util::{hex_to_vec, u8s_to_u16, u8s_to_u32};

    #[test]
    fn test_hex_to_vec() {
        assert_eq!(hex_to_vec("00 00 01 00 BA 3C 2B A9").unwrap(),
                  [0x00_u8, 0x00_u8, 0x01_u8, 0x00_u8,
                   0xBA_u8, 0x3C_u8, 0x2B_u8, 0xA9_u8]);

        match hex_to_vec("invalid HEX string") {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn test_u8s_to_u16() {
        let first = 0xAB_u8;
        let second = 0xCD_u8;
        assert_eq!(u8s_to_u16(first, second), 0xABCD_u16);
    }

    #[test]
    fn test_u8s_to_u32() {
        let first = 0x01_u8;
        let second = 0x23_u8;
        let third = 0x45_u8;
        let fourth = 0x67_u8;
        assert_eq!(u8s_to_u32(first, second, third, fourth), 0x01234567_u32);
    }

}
