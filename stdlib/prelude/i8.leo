circuit i8 {
    function to_bits_le(self) -> [bool; 8] {
        return [false; 8];
    }

    function from_bits_le(bits: [bool; 8]) -> i8 {
        return 0i8;
    }

    function to_bytes_le(self) -> [i8; 8] {
        return [0i8; 8];
    }

    function from_bytes_le(bytes: [i8; 8]) -> i8 {
        return 0i8;
    }
}