circuit group {
    @CoreFunction(group_to_bits_le)
    function to_bits_le(self) -> [bool; 506];

    @CoreFunction(group_from_bits_le)
    function from_bits_le(bits: [bool; 506]) -> group;

    @CoreFunction(group_to_bytes_le)
    function to_bytes_le(self) -> [u8; 64];

    @CoreFunction(group_from_bytes_le)
    function from_bytes_le(bytes: [u8; 64]) -> group;

    @CoreFunction(group_to_bits_be)
    function to_bits_be(self) -> [bool; 506];

    @CoreFunction(group_from_bits_be)
    function from_bits_be(bits: [bool; 506]) -> group;

    @CoreFunction(group_to_bytes_be)
    function to_bytes_be(self) -> [u8; 64];

    @CoreFunction(group_from_bytes_be)
    function from_bytes_be(bytes: [u8; 64]) -> group;
}