circuit u16 {
    function to_bits(self) -> [bool; 16] {
        return [false; 16];
    }

    function from_bits(bits: [bool; 16]) -> u16 {
        return 0u16;
    }

    function to_bytes(self) -> [u16; 2] {
        return [0u16; 2];
    }

    function from_bytes(bytes: [u16; 2]) -> u16 {
        return 0u16;
    }
}