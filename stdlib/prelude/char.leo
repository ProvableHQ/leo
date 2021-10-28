circuit char {
    function to_bits_le(self) -> [bool; 253] {
        return [false; 253];
    }

    function from_bits_le(bits: [bool; 253]) -> char {
        return 'a';
    }

    function to_bytes_le(self) -> [u8; 32] {
        return [0u8; 32];
    }

    function from_bytes_le(bytes: [u8; 32]) -> char {
        return 'a';
    }

    function to_bits_be(self) -> [bool; 253] {
        return [false; 253];
    }

    function from_bits_be(bits: [bool; 253]) -> char {
        return 'a';
    }

    function to_bytes_be(self) -> [u8; 32] {
        return [0u8; 32];
    }

    function from_bytes_be(bytes: [u8; 32]) -> char {
        return 'a';
    }
}