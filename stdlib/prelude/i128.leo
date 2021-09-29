circuit i128 {
    function to_bits_le(self) -> [bool; 128] {
        return [false; 128];
    }

    function from_bits_le(bits: [bool; 128]) -> i128 {
        return 0i128;
    }

    function to_bytes_le(self) -> [i128; 128] {
        return [0i128; 128];
    }

    function from_bytes_le(bytes: [i128; 128]) -> i128 {
        return 0i128;
    }
}