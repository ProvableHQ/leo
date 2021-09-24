circuit i16 {
    function to_bits(self) -> [bool; 16] {
        return [false; 16];
    }

    function from_bits(bits: [bool; 16]) -> i16 {
        return 0i16;
    }

    function to_bytes(self) -> [i16; 16] {
        return [0i16; 16];
    }

    function from_bytes(bytes: [i16; 16]) -> i16 {
        return 0i16;
    }
}