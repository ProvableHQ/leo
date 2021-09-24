circuit bool {
    function to_bits(self) -> [bool; 8] {
        return [false; 8];
    }

    function from_bits(bits: [bool; 8]) -> bool {
        return false;
    }

    function to_bytes(self) -> [u8; 8] {
        return [0u8; 8];
    }

    function from_bytes(bytes: [u8; 8]) -> bool {
        return false;
    }
}