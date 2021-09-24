circuit u128 {
    function to_bits(self) -> [bool; 128] {
        return [false; 128];
    }

    function from_bits(bits: [bool; 128]) -> u128 {
        return 0u128;
    }

    function to_bytes(self) -> [u128; 16] {
        return [0u128; 16];
    }

    function from_bytes(bytes: [u128; 16]) -> u128 {
        return 0u128;
    }
}