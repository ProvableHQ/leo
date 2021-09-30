circuit u16 {
    function to_bits_le(self) -> [bool; 16] {
        return [false; 16];
    }

    function from_bits_le(bits: [bool; 16]) -> u16 {
        return 0u16;
    }

    function to_bytes_le(self) -> [u8; 2] {
        return [0u8; 2];
    }

    function from_bytes_le(bytes: [u8; 2]) -> u16 {
        return 0u16;
    }
}