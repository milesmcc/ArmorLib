/// This file defines several low-level utility functions for general use by ArmorLib. You can
/// use these functions in your own program, but they are designed for internal use.

use std::num::ParseIntError;

/// A utility function that converts a string of space-separated hexadecimal bytes into a `Vec<u8>`
/// for use by preprocessors, scan modules, and other programs.
///
/// # Examples
///
/// ```rust
/// use armorlib::util;
/// assert_eq!(
///     util::hex_to_vec("00 00 01 00 BA 3C 2B A9").unwrap(),
///     [
///         0x00_u8, 0x00_u8, 0x01_u8, 0x00_u8, 0xBA_u8, 0x3C_u8, 0x2B_u8, 0xA9_u8
///     ]
/// );
///
/// match util::hex_to_vec("invalid HEX string") {
///     Ok(_) => assert!(false),
///     Err(_) => assert!(true),
/// }
/// ```
#[allow(dead_code)]
pub fn hex_to_vec(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut bytes: Vec<u8> = Vec::new();
    for byte in String::from(hex).to_uppercase().split(" ") {
        bytes.push(u8::from_str_radix(byte, 16)?);
    }
    Ok(bytes)
}

/// A utility function that, given two `u8`s, uses a bit shift operation and a binary `OR`
/// operation to return a `u16`. The first 8 bits of this `u16` are those of the argument `first`,
/// while the second 8 bits are those of the argument `second`.
///
/// # Examples
///
/// ```rust
/// use armorlib::util;
/// let first = 0x12_u8;
/// let second = 0xAB_u8;
/// assert_eq!(util::u8s_to_u16(first, second), 0x12AB_u16);
/// ```
pub fn u8s_to_u16(first: u8, second: u8) -> u16 {
    let first: u16 = (first as u16) << 8;
    let second: u16 = second as u16;

    first | second
}

/// A utility function that, given four `u8`s, uses a bit shift operation and a binary `OR`
/// operation to return a `u32`. The first 8 bits of this `u32` are those of the argument `first`,
/// the second 8 bits are those of the argument `second`, the third eight bits from `third`, and
/// the final 8 bits from `fourth`.
///
/// # Examples
///
/// ```rust
/// use armorlib::util;
/// let first = 0x12_u8;
/// let second = 0xAB_u8;
/// let third = 0xDE_u8;
/// let fourth = 0x53_u8;
/// assert_eq!(util::u8s_to_u32(first, second, third, fourth), 0x12ABDE53_u32);
/// ```
pub fn u8s_to_u32(first: u8, second: u8, third: u8, fourth: u8) -> u32 {
    let first: u32 = (first as u32) << 24;
    let second: u32 = (second as u32) << 16;
    let third: u32 = (third as u32) << 8;
    let fourth: u32 = fourth as u32;

    first | second | third | fourth
}

#[cfg(test)]
mod tests {
    use util::{hex_to_vec, u8s_to_u16, u8s_to_u32};

    #[test]
    fn test_hex_to_vec() {
        assert_eq!(
            hex_to_vec("00 00 01 00 BA 3C 2B A9").unwrap(),
            [
                0x00_u8, 0x00_u8, 0x01_u8, 0x00_u8, 0xBA_u8, 0x3C_u8, 0x2B_u8, 0xA9_u8
            ]
        );

        match hex_to_vec("invalid HEX string") {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
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
