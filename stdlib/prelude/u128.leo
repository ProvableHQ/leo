circuit u128 {
    function to_bits_le(self) -> [bool; 128] {
        return [false; 128];
    }

    function from_bits_le(bits: [bool; 128]) -> u128 {
        return 0u128;
    }

    function to_bytes_le(self) -> [u128; 16] {
        return [0u128; 16];
    }

    function from_bytes_le(bytes: [u128; 16]) -> u128 {
        return 0u128;
    }
}