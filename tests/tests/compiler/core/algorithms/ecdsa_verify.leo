/*
namespace: Compile
expectation: Pass
*/

struct Message {
    data: u64,
    nonce: u32,
}

program test.aleo {
    // Test all ECDSA verification variants with correct types
    // Note: Digest variants require [u8; 32], hash variants require byte-aligned types
    fn main(
        sig: [u8; 65],
        addr: [u8; 33],       // For non-eth variants
        addr_eth: [u8; 20],   // For eth variants
        digest: [u8; 32],     // For digest variants
        msg_u32: u32,
        msg_u64: u64,
        msg_u128: u128,
        msg_array: [u32; 2],
        msg_array2: [u64; 4]
    ) -> Final {
        return final { finalize(sig, addr, addr_eth, digest, msg_u32, msg_u64, msg_u128, msg_array, msg_array2); };
    }
}

final fn finalize(
    sig: [u8; 65],
    addr: [u8; 33],
    addr_eth: [u8; 20],
    digest: [u8; 32],
    msg_u32: u32,
    msg_u64: u64,
    msg_u128: u128,
    msg_array: [u32; 2],
    msg_array2: [u64; 4]
) {
    // Digest variants (require [u8; 32])
    let v1: bool = ECDSA::verify_digest(sig, addr, digest);
    let v2: bool = ECDSA::verify_digest_eth(sig, addr_eth, digest);

    // Keccak256 variants
    let v3: bool = ECDSA::verify_keccak256(sig, addr, msg_u128);
    let v4: bool = ECDSA::verify_keccak256_raw(sig, addr, msg_u32);
    let v5: bool = ECDSA::verify_keccak256_eth(sig, addr_eth, msg_array);

    // Keccak384 variants
    let v6: bool = ECDSA::verify_keccak384(sig, addr, 42u32);
    let v7: bool = ECDSA::verify_keccak384_raw(sig, addr, msg_u64);
    let v8: bool = ECDSA::verify_keccak384_eth(sig, addr_eth, msg_u128);

    // Keccak512 variants
    let v9: bool = ECDSA::verify_keccak512(sig, addr, msg_array2);
    let v10: bool = ECDSA::verify_keccak512_raw(sig, addr, msg_u128);
    let v11: bool = ECDSA::verify_keccak512_eth(sig, addr_eth, msg_array);

    // SHA3-256 variants
    let v12: bool = ECDSA::verify_sha3_256(sig, addr, msg_array);
    let v13: bool = ECDSA::verify_sha3_256_raw(sig, addr, 100u64);
    let v14: bool = ECDSA::verify_sha3_256_eth(sig, addr_eth, msg_u32);

    // SHA3-384 variants
    let v15: bool = ECDSA::verify_sha3_384(sig, addr, msg_array2);
    let v16: bool = ECDSA::verify_sha3_384_raw(sig, addr, msg_array);
    let v17: bool = ECDSA::verify_sha3_384_eth(sig, addr_eth, 200u128);

    // SHA3-512 variants
    let v18: bool = ECDSA::verify_sha3_512(sig, addr, msg_u64);
    let v19: bool = ECDSA::verify_sha3_512_raw(sig, addr, msg_array);
    let v20: bool = ECDSA::verify_sha3_512_eth(sig, addr_eth, msg_u128);

    // Assert all executed (prevent DCE)
    assert_eq(v1, v1);
    assert_eq(v2, v2);
    assert_eq(v3, v3);
    assert_eq(v4, v4);
    assert_eq(v5, v5);
    assert_eq(v6, v6);
    assert_eq(v7, v7);
    assert_eq(v8, v8);
    assert_eq(v9, v9);
    assert_eq(v10, v10);
    assert_eq(v11, v11);
    assert_eq(v12, v12);
    assert_eq(v13, v13);
    assert_eq(v14, v14);
    assert_eq(v15, v15);
    assert_eq(v16, v16);
    assert_eq(v17, v17);
    assert_eq(v18, v18);
    assert_eq(v19, v19);
    assert_eq(v20, v20);
}
