#[macro_use]
extern crate bitflags;

bitflags! {
    struct Flags: u8 {
        const ONE_BYTE = 248;
        const TWO_BYTES = 249;
        const THREE_BYTES = 250;
        const FOUR_BYTES = 251;
        const FIVE_BYTES = 252;
        const SIX_BYTES = 253;
        const SEVEN_BYTES = 254;
        const EIGHT_BYTES = 255;
    }
}

#[no_mangle]
pub extern "C" fn decode(bytes: &[u8]) -> u64 {
    let bits = Flags::from_bits(bytes[0]).unwrap();

    match bits {
        Flags::ONE_BYTE => bytes[1] as u64,
        Flags::TWO_BYTES => ((bytes[1] as u64) << 8) | (bytes[2] as u64),
        Flags::THREE_BYTES => {
            ((bytes[1] as u64) << 16) | ((bytes[2] as u64) << 8) | (bytes[3] as u64)
        }
        Flags::FOUR_BYTES => {
            ((bytes[1] as u64) << 16)
                | ((bytes[2] as u64) << 16)
                | ((bytes[3] as u64) << 8)
                | (bytes[4] as u64)
        }
        Flags::FIVE_BYTES => {
            ((bytes[1] as u64) << 32)
                | ((bytes[2] as u64) << 24)
                | ((bytes[3] as u64) << 16)
                | ((bytes[4] as u64) << 8)
                | (bytes[5] as u64)
        }
        Flags::SIX_BYTES => {
            ((bytes[1] as u64) << 40)
                | ((bytes[2] as u64) << 32)
                | ((bytes[3] as u64) << 24)
                | ((bytes[4] as u64) << 16)
                | ((bytes[5] as u64) << 8)
                | (bytes[6] as u64)
        }
        Flags::SEVEN_BYTES => {
            ((bytes[1] as u64) << 48)
                | ((bytes[2] as u64) << 40)
                | ((bytes[3] as u64) << 32)
                | ((bytes[4] as u64) << 24)
                | ((bytes[5] as u64) << 16)
                | ((bytes[6] as u64) << 8)
                | (bytes[7] as u64)
        }
        Flags::EIGHT_BYTES => {
            ((bytes[1] as u64) << 56)
                | ((bytes[2] as u64) << 48)
                | ((bytes[3] as u64) << 40)
                | ((bytes[4] as u64) << 32)
                | ((bytes[5] as u64) << 24)
                | ((bytes[6] as u64) << 16)
                | ((bytes[7] as u64) << 8)
                | (bytes[8] as u64)
        }
        _ => bytes[0] as u64,
    }
}

#[cfg(test)]
mod tests {
    use crate::decode;

    #[test]
    fn decode_zero() {
        let bytes = [0];
        assert_eq!(decode(&bytes), 0);
    }
    #[test]
    fn decode_eight_byte_number() {
        let bytes = [0xFF, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x1A, 0x1B];
        let result = decode(&bytes);
        println!("{:X}", result);
        assert_eq!(result,  0x0A0B0C0D0E0F1A1B);
    }
}
