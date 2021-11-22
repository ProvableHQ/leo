struct i32 {
    @CoreFunction(i32_to_bits_le)
    function to_bits_le(self) -> [bool; 32] {
        return [false; 32];
    }

    @CoreFunction(i32_from_bits_le)
    function from_bits_le(bits: [bool; 32]) -> i32 {
        return 0i32;
    }

    @CoreFunction(i32_to_bytes_le)
    function to_bytes_le(self) -> [u8; 4] {
        return [0u8; 4];
    }

    @CoreFunction(i32_from_bytes_le)
    function from_bytes_le(bytes: [u8; 4]) -> i32 {
        return 0i32;
    }

    @CoreFunction(i32_to_bits_be)
    function to_bits_be(self) -> [bool; 32] {
        return [false; 32];
    }

    @CoreFunction(i32_from_bits_be)
    function from_bits_be(bits: [bool; 32]) -> i32 {
        return 0i32;
    }

    @CoreFunction(i32_to_bytes_be)
    function to_bytes_be(self) -> [u8; 4] {
        return [0u8; 4];
    }

    @CoreFunction(i32_from_bytes_be)
    function from_bytes_be(bytes: [u8; 4]) -> i32 {
        return 0i32;
    }
}