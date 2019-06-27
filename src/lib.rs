extern crate byteorder;
use byteorder::{BigEndian, ByteOrder};

pub enum ByteNumber {
    One = 248,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[no_mangle]
pub extern "C" fn decode(bytes: &[u8]) -> u64 {
    let first_byte = bytes[0];
    if first_byte >= ByteNumber::One as u8 {
        let num_bytes = first_byte - ByteNumber::One as u8 + 1;
        BigEndian::read_uint(&bytes[1..], num_bytes as usize)
    } else {
        first_byte as u64
    }
}

#[no_mangle]
pub extern "C" fn encode<'a>(num: u64, bytes: &'a mut [u8]) -> &'a [u8] {
    if num >= ByteNumber::One as u64 {
        let mut num_bytes = 0;
        let mut j = num;
        while j > 0 {
            j = j >> 8;
            num_bytes += 1;
        }

        bytes[0] = ByteNumber::One as u8 - 1 + num_bytes;

        BigEndian::write_uint(&mut bytes[1..], num, num_bytes as usize);
        &bytes[0..num_bytes as usize + 1]
    } else {
        bytes[0] = num as u8;
        &bytes[0..1]
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn encode_zero() {
        let mut buff = [1, 0];
        let result = encode(0, &mut buff);
        assert_eq!(result[0], 0);
        assert_eq!(result.len(), 1);
    }
    #[test]
    fn encode_one_byte_number() {
        let num = 0xFE;
        let expected = [ByteNumber::One as u8, 0xFE];
        let mut buff = [0xFF; 9];
        let result = encode(num, &mut buff);
        assert_eq!(result, expected);
        assert_eq!(result.len(), 2);
    }
    #[test]
    fn encode_two_byte_number() {
        let num = 0x0B0A;
        let expected = [ByteNumber::Two as u8, 0x0B, 0x0A];
        let mut buff = [0xFF; 9];
        let result = encode(num, &mut buff);
        assert_eq!(result, expected);
        assert_eq!(result.len(), 3);
    }
    #[test]
    fn encode_three_byte_number() {
        let num = 0x0C0B0A;
        let expected = [ByteNumber::Three as u8, 0x0C, 0x0B, 0x0A];
        let mut buff = [0xFF; 9];
        let result = encode(num, &mut buff);
        assert_eq!(result, expected);
        assert_eq!(result.len(), 4);
    }
    #[test]
    fn encode_four_byte_number() {
        let num = 0x0D0C0B0A;
        let expected = [ByteNumber::Four as u8, 0x0D, 0x0C, 0x0B, 0x0A];
        let mut buff = [0xFF; 9];
        let result = encode(num, &mut buff);
        assert_eq!(result, expected);
        assert_eq!(result.len(), 5);
    }
    #[test]
    fn encode_five_byte_number() {
        let num = 0x0E0D0C0B0A;
        let expected = [ByteNumber::Five as u8, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A];
        let mut buff = [0xFF; 9];
        let result = encode(num, &mut buff);
        assert_eq!(result, expected);
        assert_eq!(result.len(), 6);
    }
    #[test]
    fn encode_six_byte_number() {
        let num = 0x0F0E0D0C0B0A;
        let expected = [ByteNumber::Six as u8, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A];
        let mut buff = [0xFF; 9];
        let result = encode(num, &mut buff);
        assert_eq!(result, expected);
        assert_eq!(result.len(), 7);
    }
    #[test]
    fn encode_seven_byte_number() {
        let num = 0x1A0F0E0D0C0B0A;
        let expected = [
            ByteNumber::Seven as u8,
            0x1A,
            0x0F,
            0x0E,
            0x0D,
            0x0C,
            0x0B,
            0x0A,
        ];
        let mut buff = [0xFF; 9];
        let result = encode(num, &mut buff);
        assert_eq!(result, expected);
        assert_eq!(result.len(), 8);
    }
    #[test]
    fn encode_eight_byte_number() {
        let num = 0x1B1A0F0E0D0C0B0A;
        let expected = [
            ByteNumber::Eight as u8,
            0x1B,
            0x1A,
            0x0F,
            0x0E,
            0x0D,
            0x0C,
            0x0B,
            0x0A,
        ];
        let mut buff = [0xFF; 9];
        let result = encode(num, &mut buff);
        assert_eq!(result, expected);
        assert_eq!(result.len(), 9);
    }
    #[test]
    fn decode_zero() {
        let bytes = [0x0A];
        assert_eq!(decode(&bytes), 0x0A);
    }
    #[test]
    fn decode_one_byte_number() {
        let bytes = [ByteNumber::One as u8, 0xFE];
        let result = decode(&bytes);
        assert_eq!(result, 0xFE);
    }
    #[test]
    fn decode_two_byte_number() {
        let bytes = [ByteNumber::Two as u8, 0x0B, 0x0A];
        let result = decode(&bytes);
        assert_eq!(result, 0x0B0A);
    }
    #[test]
    fn decode_three_byte_number() {
        let bytes = [ByteNumber::Three as u8, 0x0C, 0x0B, 0x0A];
        let result = decode(&bytes);
        assert_eq!(result, 0x0C0B0A);
    }
    #[test]
    fn decode_four_byte_number() {
        let bytes = [ByteNumber::Four as u8, 0x0D, 0x0C, 0x0B, 0x0A];
        let result = decode(&bytes);
        assert_eq!(result, 0x0D0C0B0A);
    }
    #[test]
    fn decode_five_byte_number() {
        let bytes = [ByteNumber::Five as u8, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A];
        let result = decode(&bytes);
        assert_eq!(result, 0x0E0D0C0B0A);
    }
    #[test]
    fn decode_six_byte_number() {
        let bytes = [ByteNumber::Six as u8, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A];
        let result = decode(&bytes);
        assert_eq!(result, 0x0F0E0D0C0B0A);
    }
    #[test]
    fn decode_seven_byte_number() {
        let bytes = [
            ByteNumber::Seven as u8,
            0x1A,
            0x0F,
            0x0E,
            0x0D,
            0x0C,
            0x0B,
            0x0A,
        ];
        let result = decode(&bytes);
        assert_eq!(result, 0x1A0F0E0D0C0B0A);
    }
    #[test]
    fn decode_eight_byte_number() {
        let bytes = [
            ByteNumber::Eight as u8,
            0x1B,
            0x1A,
            0x0F,
            0x0E,
            0x0D,
            0x0C,
            0x0B,
            0x0A,
        ];
        let result = decode(&bytes);
        assert_eq!(result, 0x1B1A0F0E0D0C0B0A);
    }
    #[test]
    fn encode_decode_powers_of_2() {
        let mut bytes = [0; 9];
        (0..64).into_iter().map(|n| 1 << n).for_each(|num| {
            let encoded = encode(num, &mut bytes);
            let decoded = decode(&encoded);
            assert_eq!(decoded, num);
        });
    }
}
