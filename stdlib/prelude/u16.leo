circuit u16 {
    @CoreFunction(u16_to_bits_le)
    function to_bits_le(self) -> [bool; 16] {
        return [false; 16];
    }

    @CoreFunction(u16_from_bits_le)
    function from_bits_le(bits: [bool; 16]) -> u16 {
        return 0u16;
    }

    @CoreFunction(u16_to_bytes_le)
    function to_bytes_le(self) -> [u8; 2] {
        return [0u8; 2];
    }

    @CoreFunction(u16_from_bytes_le)
    function from_bytes_le(bytes: [u8; 2]) -> u16 {
        return 0u16;
    }

    @CoreFunction(u16_to_bits_be)
    function to_bits_be(self) -> [bool; 16] {
        return [false; 16];
    }

    @CoreFunction(u16_from_bits_be)
    function from_bits_be(bits: [bool; 16]) -> u16 {
        return 0u16;
    }

    @CoreFunction(u16_to_bytes_be)
    function to_bytes_be(self) -> [u8; 2] {
        return [0u8; 2];
    }

    @CoreFunction(u16_from_bytes_be)
    function from_bytes_be(bytes: [u8; 2]) -> u16 {
        return 0u16;
    }
}