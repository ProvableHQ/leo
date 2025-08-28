program test.aleo {
    async transition main(one: bool, two: bool, three: bool) -> Future {
        return async {
            let a0: bool = ECDSA::verify_keccak256(one, two, three);
            let a1: bool = ECDSA::verify_keccak256(one, two, three);
            let a2: bool = ECDSA::verify_keccak256_raw(one, two, three);
            let a3: bool = ECDSA::verify_keccak256_eth(one, two, three);
            let a4: bool = ECDSA::verify_keccak256_eth_raw(one, two, three);
            let a5: bool = ECDSA::verify_keccak384(one, two, three);
            let a6: bool = ECDSA::verify_keccak384_raw(one, two, three);
            let a7: bool = ECDSA::verify_keccak384_eth(one, two, three);
            let a8: bool = ECDSA::verify_keccak384_eth_raw(one, two, three);
            let a9: bool = ECDSA::verify_keccak512(one, two, three);
            let a10: bool = ECDSA::verify_keccak512_raw(one, two, three);
            let a11: bool = ECDSA::verify_keccak512_eth(one, two, three);
            let a12: bool = ECDSA::verify_keccak512_eth_raw(one, two, three);
            let a13: bool = ECDSA::verify_sha3_256(one, two, three);
            let a14: bool = ECDSA::verify_sha3_256_raw(one, two, three);
            let a15: bool = ECDSA::verify_sha3_256_eth(one, two, three);
            let a16: bool = ECDSA::verify_sha3_256_eth_raw(one, two, three);
            let a17: bool = ECDSA::verify_sha3_384(one, two, three);
            let a18: bool = ECDSA::verify_sha3_384_raw(one, two, three);
            let a19: bool = ECDSA::verify_sha3_384_eth(one, two, three);
            let a20: bool = ECDSA::verify_sha3_384_eth_raw(one, two, three);
            let a21: bool = ECDSA::verify_sha3_512(one, two, three);
            let a22: bool = ECDSA::verify_sha3_512_raw(one, two, three);
            let a23: bool = ECDSA::verify_sha3_512_eth(one, two, three);
            let a24: bool = ECDSA::verify_sha3_512_eth_raw(one, two, three);

            // To prevent removal by DCE.
            assert_eq(a0, a0);
            assert_eq(a1, a1);
            assert_eq(a2, a2);
            assert_eq(a3, a3);
            assert_eq(a4, a4);
            assert_eq(a5, a5);
            assert_eq(a6, a6);
            assert_eq(a7, a7);
            assert_eq(a8, a8);
            assert_eq(a9, a9);
            assert_eq(a10, a10);
            assert_eq(a11, a11);
            assert_eq(a12, a12);
            assert_eq(a13, a13);
            assert_eq(a14, a14);
            assert_eq(a15, a15);
            assert_eq(a16, a16);
            assert_eq(a17, a17);
            assert_eq(a18, a18);
            assert_eq(a19, a19);
            assert_eq(a20, a20);
            assert_eq(a21, a21);
            assert_eq(a22, a22);
            assert_eq(a23, a23);
            assert_eq(a24, a24);
        };
    }
}
        
 
