circuit i32 {
    function to_bits(self) -> [bool; 32] {
        return [false; 32];
    }

    function from_bits(bits: [bool; 32]) -> i32 {
        return 0i32;
    }

    function to_bytes(self) -> [i32; 32] {
        return [0i32; 32];
    }

    function from_bytes(bytes: [i32; 32]) -> i32 {
        return 0i32;
    }
}