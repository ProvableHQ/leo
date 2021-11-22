struct u128 {
    @CoreFunction(u128_to_bits_le)
    function to_bits_le(self) -> [bool; 128] {
        return [false; 128];
    }

    @CoreFunction(u128_from_bits_le)
    function from_bits_le(bits: [bool; 128]) -> u128 {
        return 0u128;
    }

    @CoreFunction(u128_to_bytes_le)
    function to_bytes_le(self) -> [u8; 16] {
        return [0u8; 16];
    }

    @CoreFunction(u128_from_bytes_le)
    function from_bytes_le(bytes: [u8; 16]) -> u128 {
        return 0u128;
    }

    @CoreFunction(u128_to_bits_be)
    function to_bits_be(self) -> [bool; 128] {
        return [false; 128];
    }

    @CoreFunction(u128_from_bits_be)
    function from_bits_be(bits: [bool; 128]) -> u128 {
        return 0u128;
    }

    @CoreFunction(u128_to_bytes_be)
    function to_bytes_be(self) -> [u8; 16] {
        return [0u8; 16];
    }

    @CoreFunction(u128_from_bytes_be)
    function from_bytes_be(bytes: [u8; 16]) -> u128 {
        return 0u128;
    }
}