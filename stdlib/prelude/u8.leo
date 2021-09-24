circuit u8 {
    function to_bits(self) -> [bool; 8] {
        return [false; 8];
    }

    function from_bits(bits: [bool; 8]) -> u8 {
        return 0u8;
    }

    function to_bytes(self) -> [u8; 1] {
        return [0u8; 1];
    }

    function from_bytes(bytes: [u8; 1]) -> u8 {
        return 0u8;
    }
}