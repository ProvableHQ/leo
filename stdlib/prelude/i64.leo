circuit i64 {
    function to_bits_le(self) -> [bool; 64] {
        return [false; 64];
    }

    function from_bits_le(bits: [bool; 64]) -> i64 {
        return 0i64;
    }

    function to_bytes_le(self) -> [i64; 64] {
        return [0i64; 64];
    }

    function from_bytes_le(bytes: [i64; 64]) -> i64 {
        return 0i64;
    }
}