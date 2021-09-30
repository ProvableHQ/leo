circuit i32 {
    function to_bits_le(self) -> [bool; 32] {
        return [false; 32];
    }

    function from_bits_le(bits: [bool; 32]) -> i32 {
        return 0i32;
    }

    function to_bytes_le(self) -> [u8; 4] {
        return [0u8; 4];
    }

    function from_bytes_le(bytes: [u8; 4]) -> i32 {
        return 0i32;
    }
}