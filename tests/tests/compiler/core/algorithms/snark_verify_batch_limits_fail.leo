/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    async transition main(
        too_many_vks: [[u8; 10]; 33],
        too_many_inputs: [[[field; 1]; 1]; 33],
        varuna_version: u8,
        proof: [u8; 256],
        overflow_vks: [[u8; 10]; 5],
        overflow_inputs: [[[field; 1]; 26]; 5],
    ) -> Future {
        return finalize(
            too_many_vks, too_many_inputs,
            varuna_version, proof,
            overflow_vks, overflow_inputs,
        );
    }

    async function finalize(
        too_many_vks: [[u8; 10]; 33],
        too_many_inputs: [[[field; 1]; 1]; 33],
        varuna_version: u8,
        proof: [u8; 256],
        overflow_vks: [[u8; 10]; 5],
        overflow_inputs: [[[field; 1]; 26]; 5],
    ) {
        // 1. Fail: 33 circuits exceeds MAX_SNARK_VERIFY_CIRCUITS (32)
        let v1: bool = Snark::verify_batch(too_many_vks, varuna_version, too_many_inputs, proof);

        // 2. Fail: 5 circuits * 26 instances = 130 exceeds MAX_SNARK_VERIFY_INSTANCES (128)
        let v2: bool = Snark::verify_batch(overflow_vks, varuna_version, overflow_inputs, proof);

        assert_eq(v1, v1);
        assert_eq(v2, v2);
    }
}
