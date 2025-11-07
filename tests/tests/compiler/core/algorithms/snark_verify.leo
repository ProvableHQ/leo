program test_snark_verify.aleo {
    @custom
    async constructor() {
        assert_eq(true, true);
    }

    async transition main(
        verifying_key: [u8; 2048], 
        inputs: [field; 2048], 
        proof: [u8; 2048]
    ) -> Future {
        return async {
            assert(Snark::verify(verifying_key, inputs, proof));
        };
    }
}
