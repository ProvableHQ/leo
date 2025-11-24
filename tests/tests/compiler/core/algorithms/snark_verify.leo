program test_snark_verify.aleo {
    @custom
    async constructor() {
        assert_eq(true, true);
    }

    async transition snark_verify(
        verifying_key: [u8; 2048], 
        inputs: [field; 2048], 
        proof: [u8; 2048]
    ) -> Future {
        return async {
            assert(Snark::verify(verifying_key, inputs, proof));
        };
    }

    async transition snark_verify_batch(
        verifying_keys: [[u8; 2048]; 2048], 
        inputs: [[[field; 2048]; 2048]; 2048], 
        proofs: [[u8; 2048]; 2048]
    ) -> Future {
        return async {
            assert(Snark::verify_batch(verifying_keys, inputs, proofs));
        };
    }
}
