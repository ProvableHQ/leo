circuit u32 {
    function to_bits_le(self) -> [bool; 32] {
        return [false; 32];
    }

    function from_bits_le(bits: [bool; 32]) -> u32 {
        return 0u32;
    }

    function to_bytes_le(self) -> [u32; 4] {
        return [0u32; 4];
    }

    function from_bytes_le(bytes: [u32; 4]) -> u32 {
        return 0u32;
    }
}