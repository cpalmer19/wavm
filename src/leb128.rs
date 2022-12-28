const CONTINUATION_BIT: u8 = 0b1000_0000;
const SIGN_BIT: u8 = 0b0100_0000;
const LOW_BITS: u8 = 0b0111_1111;
const SIZE: usize = 64;

pub fn read_leb128(bytecode: &[u8], index: usize) -> (i64, usize) {
    let mut result = 0;
    let mut byte_count: usize = 0;

    loop {
        let byte = bytecode[index + byte_count];

        let low_bits = (byte & LOW_BITS) as i64;
        let shift = 7 * byte_count;
        result |= low_bits << shift;

        byte_count += 1;

        if byte & CONTINUATION_BIT == 0 {
            if shift < SIZE && (SIGN_BIT & byte) == SIGN_BIT {
                // Sign extend the result.
                result |= !0 << (shift + 7);
            }
            return (result, byte_count);
        }
    }
}

mod tests {
    #![allow(unused_imports)]
    // Not sure why the compiler thinks this import is unused
    use super::read_leb128;

    #[test]
    fn test_10() {
        let a: [u8; 1] = [0x0a];

        let (num, size) = read_leb128(&a, 0);
        assert_eq!(10, num);
        assert_eq!(1, size);
    }

    #[test]
    fn test_10_large_slice() {
        let a: [u8; 5] = [0xff, 0xff, 0x0a, 0xff, 0xff];

        let (num, size) = read_leb128(&a, 2);
        assert_eq!(10, num);
        assert_eq!(1, size);
    }

    #[test]
    fn test_10_neg() {
        let a: [u8; 1] = [0x76];

        let (num, size) = read_leb128(&a, 0);
        assert_eq!(-10, num);
        assert_eq!(1, size);
    }

    #[test]
    fn test_200() {
        let a: [u8; 2] = [0xc8, 0x01];
        
        let (num, size) = read_leb128(&a, 0);
        assert_eq!(200, num);
        assert_eq!(2, size);
    }

    #[test]
    fn test_200_neg() {
        let a: [u8; 2] = [0xb8, 0x7e];
        
        let (num, size) = read_leb128(&a, 0);
        assert_eq!(-200, num);
        assert_eq!(2, size);
    }

    #[test]
    fn test_200_000() {
        let a: [u8; 3] = [0xc0, 0x9a, 0x0c];
        
        let (num, size) = read_leb128(&a, 0);
        assert_eq!(200000, num);
        assert_eq!(3, size);
    }

    #[test]
    fn test_2_000_000_neg() {
        let a: [u8; 4] = [0x80, 0xf7, 0x85, 0x7f];
        
        let (num, size) = read_leb128(&a, 0);
        assert_eq!(-2000000, num);
        assert_eq!(4, size);
    }

    #[test]
    #[should_panic(expected = "index out of bounds: the len is 2 but the index is 2")]
    fn test_slice_too_small() {
        let a: [u8; 2] = [0x80, 0xf7];
        read_leb128(&a, 0);
    }
}