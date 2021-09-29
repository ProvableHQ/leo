circuit i32 {
    function to_bits_le(self) -> [bool; 32] {
        return [false; 32];
    }

    function from_bits_le(bits: [bool; 32]) -> i32 {
        return 0i32;
    }

    function to_bytes_le(self) -> [i32; 32] {
        return [0i32; 32];
    }

    function from_bytes_le(bytes: [i32; 32]) -> i32 {
        return 0i32;
    }
}