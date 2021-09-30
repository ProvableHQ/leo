circuit i128 {
    function to_bits_le(self) -> [bool; 128] {
        return [false; 128];
    }

    function from_bits_le(bits: [bool; 128]) -> i128 {
        return 0i128;
    }

    function to_bytes_le(self) -> [u8; 16] {
        return [0u8; 16];
    }

    function from_bytes_le(bytes: [u8; 16]) -> i128 {
        return 0i128;
    }
}