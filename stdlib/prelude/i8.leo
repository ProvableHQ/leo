circuit i8 {
    function to_bits(self) -> [bool; 8] {
        return [false; 8];
    }

    function from_bits(bits: [bool; 8]) -> i8 {
        return 0i8;
    }

    function to_bytes(self) -> [i8; 8] {
        return [0i8; 8];
    }

    function from_bytes(bytes: [i8; 8]) -> i8 {
        return 0i8;
    }
}