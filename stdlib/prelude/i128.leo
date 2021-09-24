circuit i128 {
    function to_bits(self) -> [bool; 128] {
        return [false; 128];
    }

    function from_bits(bits: [bool; 128]) -> i128 {
        return 0i128;
    }

    function to_bytes(self) -> [i128; 128] {
        return [0i128; 128];
    }

    function from_bytes(bytes: [i128; 128]) -> i128 {
        return 0i128;
    }
}