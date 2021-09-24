circuit i64 {
    function to_bits(self) -> [bool; 64] {
        return [false; 64];
    }

    function from_bits(bits: [bool; 64]) -> i64 {
        return 0i64;
    }

    function to_bytes(self) -> [i64; 64] {
        return [0i64; 64];
    }

    function from_bytes(bytes: [i64; 64]) -> i64 {
        return 0i64;
    }
}