#![allow(dead_code)]

pub mod op {
    macro_rules! op {
        ($op: ident, $code: tt) => {
            pub const $op: u8 = $code;
        }
    }

    // Control Flow
    op!(UNREACHABLE, 0x00);
    op!(NOP, 0x01);
    op!(BLOCK, 0x02);
    op!(LOOP, 0x03);
    op!(IF, 0x04);
    op!(ELSE, 0x05);

    op!(END, 0x0b);
    op!(BR, 0x0c);
    op!(BR_IF, 0x0d);
    op!(BR_TABLE, 0x0e);
    op!(RETURN, 0x0f);
    op!(CALL, 0x10);
    op!(CALL_INDIRECT, 0x11);

    op!(DROP, 0x1a);
    op!(SELECT, 0x1b);
    op!(SELECT_T, 0x1c);

    // Variables
    op!(LOCAL_GET, 0x20);
    op!(LOCAL_SET, 0x21);
    op!(LOCAL_TEE, 0x22);
    op!(GLOBAL_GET, 0x23);
    op!(GLOBAL_SET, 0x24);

    // Memory
    op!(I32_LOAD, 0x28);
    op!(I64_LOAD, 0x29);
    op!(F32_LOAD, 0x2a);
    op!(F64_LOAD, 0x2b);
    op!(I32_LOAD8_S, 0x2c);
    op!(I32_LOAD8_U, 0x2d);
    op!(I32_LOAD16_S, 0x2e);
    op!(I32_LOAD16_U, 0x2f);
    op!(I64_LOAD8_S, 0x30);
    op!(I64_LOAD8_U, 0x31);
    op!(I64_LOAD16_S, 0x32);
    op!(I64_LOAD16_U, 0x33);
    op!(I64_LOAD32_S, 0x34);
    op!(I64_LOAD32_U, 0x35);
    op!(I32_STORE, 0x36);
    op!(I64_STORE, 0x37);
    op!(F32_STORE, 0x38);
    op!(F64_STORE, 0x39);
    op!(I32_STORE8, 0x3a);
    op!(I32_STORE16, 0x3b);
    op!(I64_STORE8, 0x3c);
    op!(I64_STORE16, 0x3d);
    op!(I64_STORE32, 0x3e);
    op!(MEMORY_SIZE, 0x3f);
    op!(MEMORY_GROW, 0x40);

    // Constant
    op!(I32_CONST, 0x41);
    op!(I64_CONST, 0x42);
    op!(F32_CONST, 0x43);
    op!(F64_CONST, 0x44);

    // Comparison
    op!(I32_EQZ, 0x45);
    op!(I32_EQ, 0x46);
    op!(I32_NE, 0x47);
    op!(I32_LT_S, 0x48);
    op!(I32_LT_U, 0x49);
    op!(I32_GT_S, 0x4a);
    op!(I32_GT_U, 0x4b);
    op!(I32_LE_S, 0x4a);    // Should this be 0x4c?
    op!(I32_LE_U, 0x4d);
    op!(I32_GE_S, 0x4e);
    op!(I32_GE_U, 0x4f);
    op!(I64_EQZ, 0x50);
    op!(I64_EQ, 0x51);
    op!(I64_NE, 0x52);
    op!(I64_LT_S, 0x53);
    op!(I64_LT_U, 0x54);
    op!(I64_GT_S, 0x55);
    op!(I64_GT_U, 0x56);
    op!(I64_LE_S, 0x55);    // Should this be 0x57?
    op!(I64_LE_U, 0x58);
    op!(I64_GE_S, 0x59);
    op!(I64_GE_U, 0x5a);
    op!(F32_EQ, 0x5b);
    op!(F32_NE, 0x5c);
    op!(F32_LT, 0x5d);
    op!(F32_GT, 0x5e);
    op!(F32_LE, 0x5f);
    op!(F32_GE, 0x60);
    op!(F64_EQ, 0x61);
    op!(F64_NE, 0x62);
    op!(F64_LT, 0x63);
    op!(F64_GT, 0x64);
    op!(F64_LE, 0x65);
    op!(F64_GE, 0x66);

    // Arithmetic, Bitwise, and Math
    op!(I32_CLZ, 0x67);
    op!(I32_CTZ, 0x68);
    op!(I32_POPCNT, 0x69);
    op!(I32_ADD, 0x6a);
    op!(I32_SUB, 0x6b);
    op!(I32_MUL, 0x6c);
    op!(I32_DIV_S, 0x6d);
    op!(I32_DIV_U, 0x6e);
    op!(I32_REM_S, 0x6f);
    op!(I32_REM_U, 0x70);
    op!(I32_AND, 0x71);
    op!(I32_OR, 0x72);
    op!(I32_XOR, 0x73);
    op!(I32_SHL, 0x74);
    op!(I32_SHR_S, 0x75);
    op!(I32_SHR_U, 0x76);
    op!(I32_ROTL, 0x77);
    op!(I32_ROTR, 0x78);

    op!(I64_CLZ, 0x79);
    op!(I64_CTZ, 0x7a);
    op!(I64_POPCNT, 0x7b);
    op!(I64_ADD, 0x7c);
    op!(I64_SUB, 0x7d);
    op!(I64_MUL, 0x7e);
    op!(I64_DIV_S, 0x7f);
    op!(I64_DIV_U, 0x80);
    op!(I64_REM_S, 0x81);
    op!(I64_REM_U, 0x82);
    op!(I64_AND, 0x83);
    op!(I64_OR, 0x84);
    op!(I64_XOR, 0x85);
    op!(I64_SHL, 0x86);
    op!(I64_SHR_S, 0x87);
    op!(I64_SHR_U, 0x88);
    op!(I64_ROTL, 0x89);
    op!(I64_ROTR, 0x8a);

    op!(F32_ABS, 0x8b);
    op!(F32_NEG, 0x8c);
    op!(F32_CEIL, 0x8d);
    op!(F32_FLOOR, 0x8e);
    op!(F32_TRUNC, 0x8f);
    op!(F32_NEAREST, 0x90);
    op!(F32_SQRT, 0x91);
    op!(F32_ADD, 0x92);
    op!(F32_SUB, 0x93);
    op!(F32_MUL, 0x94);
    op!(F32_DIV, 0x95);
    op!(F32_MIN, 0x96);
    op!(F32_MAX, 0x97);
    op!(F32_COPYSIGN, 0x98);

    op!(F64_ABS, 0x99);
    op!(F64_NEG, 0x9a);
    op!(F64_CEIL, 0x9b);
    op!(F64_FLOOR, 0x9c);
    op!(F64_TRUNC, 0x9d);
    op!(F64_NEAREST, 0x9e);
    op!(F64_SQRT, 0x9f);
    op!(F64_ADD, 0xa0);
    op!(F64_SUB, 0xa1);
    op!(F64_MUL, 0xa2);
    op!(F64_DIV, 0xa3);
    op!(F64_MIN, 0xa4);
    op!(F64_MAX, 0xa5);
    op!(F64_COPYSIGN, 0xa6);

    // Conversion
    op!(I32_WRAP_I64, 0xa7);
    op!(I32_TRUNC_F32_S, 0xa8);
    op!(I32_TRUNC_F32_U, 0xa9);
    op!(I32_TRUNC_F64_S, 0xaa);
    op!(I32_TRUNC_F64_U, 0xab);
    op!(I64_EXTEND_I32_S, 0xac);
    op!(I64_EXTEND_I32_U, 0xad);
    op!(I64_TRUNC_F32_S, 0xae);
    op!(I64_TRUNC_F32_U, 0xaf);
    op!(I64_TRUNC_F64_S, 0xb0);
    op!(I64_TRUNC_F64_U, 0xb1);
    op!(F32_CONVERT_I32_S, 0xb2);
    op!(F32_CONVERT_I32_U, 0xb3);
    op!(F32_CONVERT_I64_S, 0xb4);
    op!(F32_CONVERT_I64_U, 0xb5);
    op!(F32_DEMOTE_F64, 0xb6);
    op!(F64_CONVERT_I32_S, 0xb7);
    op!(F64_CONVERT_I32_U, 0xb8);
    op!(F64_CONVERT_I64_S, 0xb9);
    op!(F64_CONVERT_I64_U, 0xba);
    op!(F64_PROMOTE_F32, 0xbb);
    op!(I32_REINTERPRET_F32, 0xbc);
    op!(I64_REINTERPRET_F64, 0xbd);
    op!(F32_REINTERPRET_I32, 0xbe);
    op!(F64_REINTERPRET_I64, 0xbf);
}

pub mod read {
    use super::leb128;

    pub fn read_8(bytecode: &[u8], index: usize) -> u8 {
        bytecode[index]
    }
    
    pub fn read_16(bytecode: &[u8], index: usize) -> u16 {
        let mut result: u16 = 0;
        let mut shift = 16 - 8;
        for i in 0..2 {
            result |= (bytecode[index+i] as u16) << shift;
            shift -= 8;
        }
        result
    }

    pub fn read_32(bytecode: &[u8], index: usize) -> u32 {
        let mut result: u32 = 0;
        let mut shift = 32 - 8;
        for i in 0..4 {
            result |= (bytecode[index+i] as u32) << shift;
            shift -= 8;
        }
        result
    }
    
    pub fn read_64(bytecode: &[u8], index: usize) -> u64 {
        let mut result: u64 = 0;
        let mut shift = 64 - 8;
        for i in 0..8 {
            result |= (bytecode[index+i] as u64) << shift;
            shift -= 8;
        }
        result
    }

    pub fn read_size(bytecode: &[u8], index: usize) -> (u32, usize) {
        let (val, size) = leb128::read_leb128(&bytecode, index);
        let val = unsafe { std::mem::transmute(val as i32)};
        (val, size)
    }

    pub fn read_i32(bytecode: &[u8], index: usize) -> (i32, usize) {
        let (val, size) = leb128::read_leb128(&bytecode, index);
        (val as i32, size)
    }

    pub fn read_i64(bytecode: &[u8], index: usize) -> (i64, usize) {
        leb128::read_leb128(&bytecode, index)
    }

    pub fn read_f32(bytecode: &[u8], index: usize) -> f32 {
        let valu32 = read_32(bytecode, index);
        unsafe { std::mem::transmute(valu32) }
    }

    pub fn read_f64(bytecode: &[u8], index: usize) -> f64 {
        let valu64 = read_64(bytecode, index);
        unsafe { std::mem::transmute(valu64) }
    }
}

mod leb128 {
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
            let a = vec![0x0a];
            let (num, size) = read_leb128(&a, 0);
            assert_eq!(10, num);
            assert_eq!(1, size);
        }

        #[test]
        fn test_10_large_slice() {
            let a = vec![0xff, 0xff, 0x0a, 0xff, 0xff];
            let (num, size) = read_leb128(&a, 2);
            assert_eq!(10, num);
            assert_eq!(1, size);
        }

        #[test]
        fn test_10_neg() {
            let a = vec![0x76];
            let (num, size) = read_leb128(&a, 0);
            assert_eq!(-10, num);
            assert_eq!(1, size);
        }

        #[test]
        fn test_200() {
            let a = vec![0xc8, 0x01];
            let (num, size) = read_leb128(&a, 0);
            assert_eq!(200, num);
            assert_eq!(2, size);
        }

        #[test]
        fn test_200_neg() {
            let a = vec![0xb8, 0x7e];
            let (num, size) = read_leb128(&a, 0);
            assert_eq!(-200, num);
            assert_eq!(2, size);
        }

        #[test]
        fn test_200_000() {
            let a = vec![0xc0, 0x9a, 0x0c];
            let (num, size) = read_leb128(&a, 0);
            assert_eq!(200000, num);
            assert_eq!(3, size);
        }

        #[test]
        fn test_2_000_000_neg() {
            let a = vec![0x80, 0xf7, 0x85, 0x7f];
            let (num, size) = read_leb128(&a, 0);
            assert_eq!(-2000000, num);
            assert_eq!(4, size);
        }

        #[test]
        #[should_panic(expected = "index out of bounds: the len is 2 but the index is 2")]
        fn test_slice_too_small() {
            let a = vec![0x80, 0xf7];
            read_leb128(&a, 0);
        }
    }
}