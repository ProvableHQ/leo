circuit u8 {
    function to_bits_le(self) -> [bool; 8] {
        return [false; 8];
    }

    function from_bits_le(bits: [bool; 8]) -> u8 {
        return 0u8;
    }

    function to_bytes_le(self) -> [u8; 1] {
        return [0u8; 1];
    }

    function from_bytes_le(bytes: [u8; 1]) -> u8 {
        return 0u8;
    }
}