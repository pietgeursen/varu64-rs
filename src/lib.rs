extern crate byteorder;
use byteorder::{ByteOrder, BigEndian};


const ONE_BYTE: u8 = 248;
const TWO_BYTES: u8 = 249;
const THREE_BYTES: u8 = 250;
const FOUR_BYTES: u8 = 251;
const FIVE_BYTES: u8 = 252;
const SIX_BYTES: u8 = 253;
const SEVEN_BYTES: u8 = 254;
const EIGHT_BYTES: u8 = 255;

#[no_mangle]
pub extern "C" fn decode(bytes: &[u8]) -> u64 {
    let first_byte = bytes[0];
    if first_byte >= ONE_BYTE {
        let num_bytes = first_byte - ONE_BYTE + 1;
        BigEndian::read_uint(&bytes[1..], num_bytes as usize)
    } else {
        first_byte as u64
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn decode_zero() {
        let bytes = [0x0A];
        assert_eq!(decode(&bytes), 0x0A);
    }
    #[test]
    fn decode_one_byte_number() {
        let bytes = [ONE_BYTE, 0xFE];
        let result  = decode(&bytes);
        assert_eq!(result, 0xFE);
    }
    #[test]
    fn decode_two_byte_number() {
        let bytes = [TWO_BYTES, 0x0B, 0x0A];
        let result  = decode(&bytes);
        assert_eq!(result, 0x0B0A);
    }
    #[test]
    fn decode_three_byte_number() {
        let bytes = [THREE_BYTES, 0x0C, 0x0B, 0x0A];
        let result  = decode(&bytes);
        assert_eq!(result, 0x0C0B0A);
    }
    #[test]
    fn decode_four_byte_number() {
        let bytes = [FOUR_BYTES, 0x0D, 0x0C, 0x0B, 0x0A];
        let result  = decode(&bytes);
        assert_eq!(result, 0x0D0C0B0A);
    }
    #[test]
    fn decode_five_byte_number() {
        let bytes = [FIVE_BYTES, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A];
        let result  = decode(&bytes);
        assert_eq!(result, 0x0E0D0C0B0A);
    }
    #[test]
    fn decode_six_byte_number() {
        let bytes = [SIX_BYTES, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A];
        let result  = decode(&bytes);
        assert_eq!(result, 0x0F0E0D0C0B0A);
    }
    #[test]
    fn decode_seven_byte_number() {
        let bytes = [SEVEN_BYTES, 0x1A, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A];
        let result  = decode(&bytes);
        assert_eq!(result, 0x1A0F0E0D0C0B0A);
    }
    #[test]
    fn decode_eight_byte_number() {
        let bytes = [EIGHT_BYTES, 0x1B, 0x1A, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A];
        let result  = decode(&bytes);
        assert_eq!(result, 0x1B1A0F0E0D0C0B0A);
    }
}
