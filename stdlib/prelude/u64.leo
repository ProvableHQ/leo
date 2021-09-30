circuit u64 {
    function to_bits_le(self) -> [bool; 64] {
        return [false; 64];
    }

    function from_bits_le(bits: [bool; 64]) -> u64 {
        return 0u64;
    }

    function to_bytes_le(self) -> [u8; 8] {
        return [0u8; 8];
    }

    function from_bytes_le(bytes: [u8; 8]) -> u64 {
        return 0u64;
    }
}