program test.aleo {
    fn select_u32(s: bool, a: [u32; 8], b: [u32; 8]) -> [u32; 8] {
        return s ? a : b;
    }

    fn select_bool(s: bool, a: [bool; 4], b: [bool; 4]) -> [bool; 4] {
        return s ? a : b;
    }

    fn select_field(s: bool, a: [field; 3], b: [field; 3]) -> [field; 3] {
        return s ? a : b;
    }

    fn select_nested(s: bool, a: [[u8; 2]; 3], b: [[u8; 2]; 3]) -> [[u8; 2]; 3] {
        return s ? a : b;
    }
}
