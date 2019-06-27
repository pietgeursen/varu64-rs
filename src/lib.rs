extern crate byteorder;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};


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
    if bytes[0] >= ONE_BYTE {
        let mut rdr = Cursor::new(&bytes[1..]);
        rdr.read_u64::<BigEndian>().unwrap()
    }else{
        bytes[0] as u64
    }
}

#[cfg(test)]
mod tests {
    use crate::decode;

    #[test]
    fn decode_zero() {
        let bytes = [0x0A];
        assert_eq!(decode(&bytes), 0x0A);
    }
    #[test]
    fn decode_seven_byte_number() {
        let bytes = [0xFE, 0x1A, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A];
        let result  = decode(&bytes);
        println!("{:X}", result);
        assert_eq!(result, 0x1A0F0E0D0C0B0A);
    }
    #[test]
    fn decode_eight_byte_number() {
        let bytes = [0xFF, 0x1B, 0x1A, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A];
        let result  = decode(&bytes);
        println!("{:X}", result);
        assert_eq!(result, 0x1B1A0F0E0D0C0B0A);
    }
}
