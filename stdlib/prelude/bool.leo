circuit bool {
    function to_bits_le(self) -> [bool; 1] {
        return [false; 1];
    }

    function from_bits_le(bits: [bool; 1]) -> bool {
        return false;
    }

    function to_bytes_le(self) -> [u8; 1] {
        return [0u8; 1];
    }

    function from_bytes_le(bytes: [u8; 1]) -> bool {
        return false;
    }

    function to_bits_be(self) -> [bool; 1] {
        return [false; 1];
    }

    function from_bits_be(bits: [bool; 1]) -> bool {
        return false;
    }

    function to_bytes_be(self) -> [u8; 1] {
        return [0u8; 1];
    }

    function from_bytes_be(bytes: [u8; 1]) -> bool {
        return false;
    }
}