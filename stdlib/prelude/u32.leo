circuit u32 {
    function to_bits(self) -> [bool; 32] {
        return [false; 32];
    }

    function from_bits(bits: [bool; 32]) -> u32 {
        return 0u32;
    }

    function to_bytes(self) -> [u32; 4] {
        return [0u32; 4];
    }

    function from_bytes(bytes: [u32; 4]) -> u32 {
        return 0u32;
    }
}