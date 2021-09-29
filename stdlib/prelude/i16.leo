circuit i16 {
    function to_bits_le(self) -> [bool; 16] {
        return [false; 16];
    }

    function from_bits_le(bits: [bool; 16]) -> i16 {
        return 0i16;
    }

    function to_bytes_le(self) -> [i16; 16] {
        return [0i16; 16];
    }

    function from_bytes_le(bytes: [i16; 16]) -> i16 {
        return 0i16;
    }
}