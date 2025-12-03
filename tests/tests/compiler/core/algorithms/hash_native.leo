/*
namespace: Compile
expectation: Pass
*/

program test.aleo {
    // Test native hash functions that return bit arrays
    transition main(input: u64) -> (
        bool, bool, bool, bool,  // Keccak256 native samples
        bool, bool,               // Keccak384 native samples
        bool, bool,               // Keccak512 native samples
        bool, bool,               // SHA3-256 native samples
        bool, bool,               // SHA3-384 native samples
        bool, bool,               // SHA3-512 native samples
        bool, bool                // Comparison flags
    ) {
        // Keccak256 hash_to_bits - returns [bool; 256]
        let keccak256_bits: [bool; 256] = Keccak256::hash_to_bits(input);
        let keccak256_0: bool = keccak256_bits[0u8];
        let keccak256_100: bool = keccak256_bits[100u8];
        let keccak256_200: bool = keccak256_bits[200u8];
        let keccak256_255: bool = keccak256_bits[255u8];

        // Keccak384 hash_to_bits - returns [bool; 384]
        let keccak384_bits: [bool; 384] = Keccak384::hash_to_bits(input);
        let keccak384_0: bool = keccak384_bits[0u16];
        let keccak384_383: bool = keccak384_bits[383u16];

        // Keccak512 hash_to_bits - returns [bool; 512]
        let keccak512_bits: [bool; 512] = Keccak512::hash_to_bits(input);
        let keccak512_0: bool = keccak512_bits[0u16];
        let keccak512_511: bool = keccak512_bits[511u16];

        // SHA3-256 hash_to_bits - returns [bool; 256]
        let sha3_256_bits: [bool; 256] = SHA3_256::hash_to_bits(input);
        let sha3_256_0: bool = sha3_256_bits[0u8];
        let sha3_256_255: bool = sha3_256_bits[255u8];

        // SHA3-384 hash_to_bits - returns [bool; 384]
        let sha3_384_bits: [bool; 384] = SHA3_384::hash_to_bits(input);
        let sha3_384_0: bool = sha3_384_bits[0u16];
        let sha3_384_383: bool = sha3_384_bits[383u16];

        // SHA3-512 hash_to_bits - returns [bool; 512]
        let sha3_512_bits: [bool; 512] = SHA3_512::hash_to_bits(input);
        let sha3_512_0: bool = sha3_512_bits[0u16];
        let sha3_512_511: bool = sha3_512_bits[511u16];

        // Test that native and native_raw produce different results
        let keccak256_bits_raw: [bool; 256] = Keccak256::hash_to_bits_raw(input);
        let sha3_256_bits_raw: [bool; 256] = SHA3_256::hash_to_bits_raw(input);

        return (
            keccak256_0, keccak256_100, keccak256_200, keccak256_255,
            keccak384_0, keccak384_383,
            keccak512_0, keccak512_511,
            sha3_256_0, sha3_256_255,
            sha3_384_0, sha3_384_383,
            sha3_512_0, sha3_512_511,
            keccak256_bits[0u8] != keccak256_bits_raw[0u8],
            sha3_256_bits[0u8] != sha3_256_bits_raw[0u8]
        );
    }
}
