circuit u64 {
    function to_bits(self) -> [bool; 64] {
        return [false; 64];
    }

    function from_bits(bits: [bool; 64]) -> u64 {
        return 0u64;
    }

    function to_bytes(self) -> [u64; 8] {
        return [0u64; 8];
    }

    function from_bytes(bytes: [u64; 8]) -> u64 {
        return 0u64;
    }
}