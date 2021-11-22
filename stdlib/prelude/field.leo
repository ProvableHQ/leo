struct field {
    @CoreFunction(field_to_bits_le)
    function to_bits_le(self) -> [bool; 253] {
        return [false; 253];
    }

    @CoreFunction(field_from_bits_le)
    function from_bits_le(bits: [bool; 253]) -> field {
        return 0field;
    }

    @CoreFunction(field_to_bytes_le)
    function to_bytes_le(self) -> [u8; 32] {
        return [0u8; 32];
    }

    @CoreFunction(field_from_bytes_le)
    function from_bytes_le(bytes: [u8; 32]) -> field {
        return 0field;
    }

    @CoreFunction(field_to_bits_be)
    function to_bits_be(self) -> [bool; 253] {
        return [false; 253];
    }

    @CoreFunction(field_from_bits_be)
    function from_bits_be(bits: [bool; 253]) -> field {
        return 0field;
    }

    @CoreFunction(field_to_bytes_be)
    function to_bytes_be(self) -> [u8; 32] {
        return [0u8; 32];
    }

    @CoreFunction(field_from_bytes_be)
    function from_bytes_be(bytes: [u8; 32]) -> field {
        return 0field;
    }
}