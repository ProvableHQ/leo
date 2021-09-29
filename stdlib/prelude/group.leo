circuit group {
    function to_bits_le(self) -> [bool; 8] {
        return [false; 8];
    }

    function from_bits_le(bits: [bool; 8]) -> group {
        return 0group;
    }

    function to_bytes_le(self) -> [u8; 8] {
        return [0u8; 8];
    }

    function from_bytes_le(bytes: [u8; 8]) -> group {
        return 0group;
    }
}