/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    // Test ECDSA verification with incorrect types - should fail
    fn main(
        sig_correct: [u8; 65],          // Correct signature
        sig_wrong_size: [u8; 64],       // Wrong: should be [u8; 65]
        sig_wrong_type: u64,            // Wrong: should be [u8; 65]
        addr_wrong_size_1: [u8; 20],    // Wrong for non-eth: should be [u8; 33]
        addr_wrong_size_2: [u8; 33],    // Wrong for eth: should be [u8; 20]
        addr_wrong_elem: [u32; 33],     // Wrong element type: should be u8
        digest_wrong_size: [u8; 31],    // Wrong for digest: should be [u8; 32]
        msg: u64
    ) -> Final {
        return final { finalize(sig_correct, sig_wrong_size, sig_wrong_type, addr_wrong_size_1, addr_wrong_size_2, addr_wrong_elem, digest_wrong_size, msg); };
    }
}

final fn finalize(
    sig_correct: [u8; 65],
    sig_wrong_size: [u8; 64],
    sig_wrong_type: u64,
    addr_wrong_size_1: [u8; 20],
    addr_wrong_size_2: [u8; 33],
    addr_wrong_elem: [u32; 33],
    digest_wrong_size: [u8; 31],
    msg: u64
) {
    // These should all fail with type errors:

    // Fail: signature wrong size with digest wrong size
    let v1: bool = ECDSA::verify_digest(sig_wrong_size, addr_wrong_size_2, digest_wrong_size);

    // Fail: signature wrong type
    let v2: bool = ECDSA::verify_keccak256(sig_wrong_type, addr_wrong_size_2, msg);

    // Fail: correct signature, but non-eth variant with eth-sized address
    let v3: bool = ECDSA::verify_digest(sig_correct, addr_wrong_size_1, msg);

    // Fail: correct signature, but eth variant with non-eth-sized address
    let v4: bool = ECDSA::verify_digest_eth(sig_correct, addr_wrong_size_2, msg);

    // Fail: correct signature, but address with wrong element type
    let v5: bool = ECDSA::verify_sha3_256(sig_correct, addr_wrong_elem, msg);

    // Fail: correct signature, but hash eth variant with wrong address size
    let v6: bool = ECDSA::verify_keccak256_eth(sig_correct, addr_wrong_size_2, msg);

    // Fail: correct signature, but hash non-eth variant with wrong address size
    let v7: bool = ECDSA::verify_sha3_512(sig_correct, addr_wrong_size_1, msg);

    // Fail: wrong signature size with wrong address
    let v8: bool = ECDSA::verify_keccak384(sig_wrong_size, addr_wrong_size_1, msg);

    // Fail: correct signature but wrong digest size for digest variant
    let v9: bool = ECDSA::verify_digest(sig_correct, addr_wrong_size_2, digest_wrong_size);

    // Fail: correct signature but wrong digest size for digest_eth variant
    let v10: bool = ECDSA::verify_digest_eth(sig_correct, addr_wrong_size_1, digest_wrong_size);

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
}
