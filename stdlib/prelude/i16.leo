struct i16 {
    @CoreFunction(i16_to_bits_le)
    function to_bits_le(self) -> [bool; 16] {
        return [false; 16];
    }

    @CoreFunction(i16_from_bits_le)
    function from_bits_le(bits: [bool; 16]) -> i16 {
        return 0i16;
    }

    @CoreFunction(i16_to_bytes_le)
    function to_bytes_le(self) -> [u8; 2] {
        return [0u8; 2];
    }

    @CoreFunction(i16_from_bytes_le)
    function from_bytes_le(bytes: [u8; 2]) -> i16 {
        return 0i16;
    }

    @CoreFunction(i16_to_bits_be)
    function to_bits_be(self) -> [bool; 16] {
        return [false; 16];
    }

    @CoreFunction(i16_from_bits_be)
    function from_bits_be(bits: [bool; 16]) -> i16 {
        return 0i16;
    }

    @CoreFunction(i16_to_bytes_be)
    function to_bytes_be(self) -> [u8; 2] {
        return [0u8; 2];
    }

    @CoreFunction(i16_from_bytes_be)
    function from_bytes_be(bytes: [u8; 2]) -> i16 {
        return 0i16;
    }
}