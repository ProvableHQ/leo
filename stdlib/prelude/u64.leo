circuit u64 {
    @CoreFunction(u64_to_bits_le)
    function to_bits_le(self) -> [bool; 64];

    @CoreFunction(u64_from_bits_le)
    function from_bits_le(bits: [bool; 64]) -> u64;

    @CoreFunction(u64_to_bytes_le)
    function to_bytes_le(self) -> [u8; 8];

    @CoreFunction(u64_from_bytes_le)
    function from_bytes_le(bytes: [u8; 8]) -> u64;

    @CoreFunction(u64_to_bits_be)
    function to_bits_be(self) -> [bool; 64];

    @CoreFunction(u64_from_bits_be)
    function from_bits_be(bits: [bool; 64]) -> u64;

    @CoreFunction(u64_to_bytes_be)
    function to_bytes_be(self) -> [u8; 8];

    @CoreFunction(u64_from_bytes_be)
    function from_bytes_be(bytes: [u8; 8]) -> u64;
}