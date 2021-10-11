circuit group {
    function to_bits_le(self) -> [bool; 506] {
        return [false; 506];
    }

    function from_bits_le(bits: [bool; 506]) -> group {
        return 0group;
    }

    function to_bytes_le(self) -> [u8; 64] {
        return [0u8; 64];
    }

    function from_bytes_le(bytes: [u8; 64]) -> group {
        return 0group;
    }

    function to_bits_be(self) -> [bool; 506] {
        return [false; 506];
    }

    function from_bits_be(bits: [bool; 506]) -> group {
        return 0group;
    }

    function to_bytes_be(self) -> [u8; 64] {
        return [0u8; 64];
    }

    function from_bytes_be(bytes: [u8; 64]) -> group {
        return 0group;
    }
}